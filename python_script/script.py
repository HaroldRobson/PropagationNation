#!/usr/bin/env python3
"""
Populate species_of_plants table from Perenual API
Rotates through API keys when rate limited
"""

import requests
import psycopg2
import json
import time
from typing import Optional, Dict, List

# Configuration
DB_URI = "postgresql://johnjohnson@localhost:5432/propagationnation_development"
API_KEYS = [
    "sk-Um7K68fcb2e61e99213109",
    "sk-Cinp68fcb53c1f62a13111",
    "sk-Eyl868fcb57cc267713112",
    "sk-SuR968fcb5c02a0c313113",
    "sk-IvR668fcb5fd6db7913115",
]
BASE_URL = "https://perenual.com/api/v2/species/details"

current_key_index = 0


def get_next_api_key() -> str:
    """Rotate to next API key"""
    global current_key_index
    current_key_index = (current_key_index + 1) % len(API_KEYS)
    print(f"\n🔄 Rotating to API key #{current_key_index + 1}")
    return API_KEYS[current_key_index]


def build_care_instructions(data: Dict) -> Optional[str]:
    """Build care instructions from API response data"""
    instructions = []
    
    # Watering info
    if data.get("watering"):
        watering_text = f"Watering: {data['watering']}"
        if data.get("watering_general_benchmark") and data["watering_general_benchmark"].get("value"):
            watering_text += f" (every {data['watering_general_benchmark']['value']} {data['watering_general_benchmark'].get('unit', 'days')})"
        instructions.append(watering_text)
    
    # Sunlight
    if data.get("sunlight"):
        sunlight_list = data["sunlight"] if isinstance(data["sunlight"], list) else [data["sunlight"]]
        instructions.append(f"Sunlight: {', '.join(sunlight_list)}")
    
    # Care level
    if data.get("care_level"):
        instructions.append(f"Care Level: {data['care_level']}")
    
    # Maintenance
    if data.get("maintenance"):
        instructions.append(f"Maintenance: {data['maintenance']}")
    
    # Growth rate
    if data.get("growth_rate"):
        instructions.append(f"Growth Rate: {data['growth_rate']}")
    
    # Pruning
    if data.get("pruning_month"):
        pruning_months = data["pruning_month"] if isinstance(data["pruning_month"], list) else [data["pruning_month"]]
        if pruning_months:
            instructions.append(f"Pruning Months: {', '.join(pruning_months[:6])}")  # Limit to avoid duplicates
    
    # Toxicity warnings
    toxicity = []
    if data.get("poisonous_to_humans"):
        toxicity.append("toxic to humans")
    if data.get("poisonous_to_pets"):
        toxicity.append("toxic to pets")
    if toxicity:
        instructions.append(f"⚠️ WARNING: {', '.join(toxicity)}")
    
    # Additional notes
    if data.get("indoor"):
        instructions.append("✓ Suitable for indoor growing")
    if data.get("drought_tolerant"):
        instructions.append("✓ Drought tolerant")
    
    return "\n".join(instructions) if instructions else None


def build_photo_json(default_image: Optional[Dict]) -> Optional[str]:
    """Build JSON object with photo URLs"""
    if not default_image:
        return None
    
    photo_data = {}
    if default_image.get("original_url"):
        photo_data["original"] = default_image["original_url"]
    if default_image.get("regular_url"):
        photo_data["regular"] = default_image["regular_url"]
    if default_image.get("medium_url"):
        photo_data["medium"] = default_image["medium_url"]
    
    return json.dumps(photo_data) if photo_data else None


def fetch_species_data(species_id: int, api_key: str) -> Optional[Dict]:
    """Fetch species data from Perenual API"""
    url = f"{BASE_URL}/{species_id}?key={api_key}"
    
    try:
        response = requests.get(url, timeout=10)
        
        # Check for rate limiting or other errors
        if response.status_code == 429:
            print(f"⚠️  Rate limited on species_id {species_id}")
            return "RATE_LIMITED"
        
        if response.status_code != 200:
            print(f"❌ Error {response.status_code} for species_id {species_id}")
            return None
        
        data = response.json()
        
        # Check if response indicates we need to upgrade (hit free tier limit)
        if isinstance(data, dict) and "message" in data:
            if "upgrade" in data["message"].lower() or "limit" in data["message"].lower():
                print(f"⚠️  API limit reached on species_id {species_id}")
                return "RATE_LIMITED"
        
        return data
        
    except requests.exceptions.RequestException as e:
        print(f"❌ Network error for species_id {species_id}: {e}")
        return None
    except json.JSONDecodeError as e:
        print(f"❌ JSON decode error for species_id {species_id}: {e}")
        return None


def insert_species(conn, data: Dict, species_id: int):
    """Insert species data into database"""
    cursor = conn.cursor()
    
    try:
        # Extract required fields
        common_name = data.get("common_name")
        scientific_name = data.get("scientific_name")
        
        # Skip if missing required fields
        if not common_name or not scientific_name:
            print(f"⚠️  Skipping species_id {species_id}: missing required fields")
            return False
        
        # Handle scientific_name as list or string
        if isinstance(scientific_name, list):
            scientific_name = scientific_name[0] if scientific_name else None
        
        if not scientific_name:
            print(f"⚠️  Skipping species_id {species_id}: empty scientific name")
            return False
        
        # Extract optional fields
        family_name = data.get("family")
        origin = data.get("origin")
        if isinstance(origin, list):
            origin = ", ".join(origin) if origin else None
        
        care_instructions = build_care_instructions(data)
        photo_json = build_photo_json(data.get("default_image"))
        
        # Insert into database
        # Note: created_at and updated_at should be handled by Loco automatically
        insert_query = """
            INSERT INTO species_of_plants 
            (id, common_name, scientific_name, family_name, care_instructions, origin, photo_url)
            VALUES (%s, %s, %s, %s, %s, %s, %s)
            ON CONFLICT (id) DO NOTHING
        """
        
        cursor.execute(insert_query, (
            species_id,
            common_name,
            scientific_name,
            family_name,
            care_instructions,
            origin,
            photo_json
        ))
        
        conn.commit()
        return True
        
    except Exception as e:
        print(f"❌ Database error for species_id {species_id}: {e}")
        conn.rollback()
        return False
    finally:
        cursor.close()


def main():
    """Main function to populate the database"""
    print("🌱 Starting plant species population from Perenual API")
    print(f"📊 Using {len(API_KEYS)} API keys")
    print("=" * 60)
    
    # Connect to database
    try:
        conn = psycopg2.connect(DB_URI)
        print("✅ Connected to database")
    except Exception as e:
        print(f"❌ Failed to connect to database: {e}")
        return
    
    current_api_key = API_KEYS[current_key_index]
    species_id = 1
    inserted_count = 0
    skipped_count = 0
    error_count = 0
    
    try:
        while species_id <= 10000:  # Perenual has 10,000+ species
            print(f"\n📍 Processing species_id: {species_id}")
            
            # Fetch data
            data = fetch_species_data(species_id, current_api_key)
            
            # Handle rate limiting
            if data == "RATE_LIMITED":
                print(f"🔄 Hit rate limit, rotating API key...")
                current_api_key = get_next_api_key()
                time.sleep(2)  # Brief pause before retry
                continue  # Retry same ID with new key
            
            # Handle other errors
            if data is None:
                error_count += 1
                species_id += 1
                time.sleep(0.5)  # Brief pause on error
                continue
            
            # Insert into database
            if insert_species(conn, data, species_id):
                inserted_count += 1
                print(f"✅ Inserted: {data.get('common_name')} (Total: {inserted_count})")
            else:
                skipped_count += 1
            
            species_id += 1
            
            # Small delay to be respectful to API
            time.sleep(0.3)
            
            # Stop if we've inserted ~500 species (roughly 5 keys * 100 each)
            if inserted_count >= 500:
                print(f"\n🎉 Reached target of 500 species!")
                break
    
    except KeyboardInterrupt:
        print(f"\n\n⚠️  Interrupted by user")
    finally:
        print("\n" + "=" * 60)
        print(f"📊 Final Statistics:")
        print(f"   Last processed ID: {species_id - 1}")
        print(f"   Successfully inserted: {inserted_count}")
        print(f"   Skipped: {skipped_count}")
        print(f"   Errors: {error_count}")
        print("=" * 60)
        
        conn.close()
        print("✅ Database connection closed")


if __name__ == "__main__":
    main()
