use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
// Modules
mod components;
use components::counter_btn::Button;
mod pages;

// Top-Level pages
use crate::pages::home::Home;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(69);
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <button on:click=move |_| set_count.set(3)
            >
            "bunting says meow: "
            {count}
        </button>
        <p>
        "double count"
        {move || count.get() * 2}
        </p>
            <App2/>
            <DynamicList/>
            <Button/>

    }
}

#[component]
pub fn App2() -> impl IntoView {
    let (count, set_count) = signal(0);
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <button on:click=move |_| set_count.update(|count: &mut i32| *count += 1)
            style="position: absolute"
        // and toggle individual CSS properties with `style:`
        class:red=move || count.get() % 2 == 1
        style:left=move || format!("{}px", count.get() + 100)
        style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
        style:max-width="400px"
        // Set a CSS variable for stylesheet use
        style=("--columns", move || count.get().to_string())
            >
            "bunting says meow: "
            {count}
        </button>
        <p>
        "double count"
        {move || count.get() * 2}
        </p>
        <progress max="50" value=move || 3 * count.get()/>
        <ProgressBar progress=count/>
        <List/>

    }
}

#[component]
pub fn ProgressBar(#[prop(default = 10)] max: i32, progress: ReadSignal<i32>) -> impl IntoView {
    view! {
        <progress
            max=max as u16
            value=progress
            />


    }
}

#[component]
pub fn List() -> impl IntoView {
    let values = vec![1, 2, 3, 4];
    view! {
        <p>{values.clone()}</p>
        <ul>
        {values.into_iter().map(|n| view! {<li>{n}</li>}).collect::<Vec<_>>()}
        </ul>
    }
}

struct ListElement {
    id: i32,
    text: String,
}

trait DisplayStruct {
    fn display(&self) -> impl IntoView;
}

impl DisplayStruct for ListElement {
    fn display(&self) -> impl IntoView {
        view! {
            <button> meow</button>
        }
    }
}

#[component]
pub fn DynamicList() -> impl IntoView {
    let (list, set_list) = signal::<Vec<(i32, i32)>>(vec![(10, 0)]);
    let (counter, set_counter) = signal(0);
    view! {
            <button on:click=move |_| {
                set_counter.update(|c: &mut i32| *c += 1);
                set_list.update(|list: &mut Vec<(i32,i32)>| list.push((10i32, counter.get() as i32)));
            }>
            "addtolist"
            </button>

    <For each=move || list.get()
        key=|x| x.1
        children=move |tuple: (i32, i32)| {
            view! {
                <button>"List Element: " {move || (tuple.0, tuple.1)}</button>
                <button on:click=move |_| {
                    set_list.update(|list| list.retain(|(_, id)| id != &tuple.1));
                }> "remove"</button>
            }
        }
    />


        }
}

#[component]
fn DynamicList2(
    /// The number of counters to begin with.
    #[prop(default = 2)]
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    // see NOTE in add_counter below re: ArcRwSignal
    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        // we use ArcRwSignal here, instead of RwSignal
        // ArcRwSignal is a reference-counted type, rather than the arena-allocated
        // signal types we've been using so far.
        // When we're creating a collection of signals like this, using ArcRwSignal
        // allows each signal to be deallocated when its row is removed.
        let sig = ArcRwSignal::new(next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=move || counters.get()
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, count)| {
                        // we can convert our ArcRwSignal to a Copy-able RwSignal
                        // for nicer DX when moving it into the view
                        let count = RwSignal::from(count);
                        view! {
                            <li>
                                <button
                                    on:click=move |_| *count.write() += 1
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters
                                            .write()
                                            .retain(|(counter_id, _)| {
                                                counter_id != &id
                                            });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
