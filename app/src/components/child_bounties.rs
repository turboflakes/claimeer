use crate::components::{
    buttons::ClaimButton,
    inputs::FilterInput,
    items::{ChildBountyItem, FilterItem},
    spinners::Spinner,
};
use crate::state::{Action, StateContext};
use claimit_common::runtimes::utils::amount_human;
use claimit_common::types::child_bounties::Filter;
use strum::IntoEnumIterator;
use yew::{function_component, html, use_context, use_state, Callback, Html};

#[function_component(ChildBountiesCard)]
pub fn child_bounties_card() -> Html {
    let state = use_context::<StateContext>().unwrap();

    if (state.network.is_initializing() || state.network.is_fetching())
        && state.child_bounties_raw.is_none()
    {
        html! {
            <div class="flex flex-col justify-center items-center h-96 p-4 md:p-6 bg-gray-50 max-w-[375px] sm:max-w-[828px] rounded-lg w-full">
                <Spinner is_visible={state.network.is_initializing() || state.network.is_fetching()} />
                {
                    if state.network.is_initializing() {
                        html! {<p class="mt-4 text-sm text-center">{state.network.is_initializing_description()}</p>}
                    } else {
                        html! {<p class="mt-4 text-sm text-center">{state.network.is_fetching_description()}</p>}
                    }
                }

            </div>
        }
    } else if state.child_bounties_raw.is_some() {
        html! {
            <div class="flex flex-col min-h-96 p-4 md:p-6 bg-gray-50  max-w-[375px] sm:max-w-[828px] rounded-lg w-full">

                <ChildBountiesTitle />

                <ChildBountiesBody />

            </div>
        }
    } else {
        html! {}
    }
}

#[function_component(ChildBountiesFilters)]
pub fn child_bounties_filters() -> Html {
    let state = use_context::<StateContext>().unwrap();

    let onclick = {
        let state = state.clone();
        Callback::from(move |e| {
            state.dispatch(Action::SetFilter(e));
        })
    };

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        let child_bountes_total = child_bounties_raw
            .into_iter()
            .filter(|(_, cb)| state.filter.check(cb))
            .count();

        html! {
            <div class="flex md:mb-4 justify-between items-center ">
                <div class="inline-flex mb-2">
                    <h3 class="md:text-lg text-gray-900 dark:text-gray-100 me-1">{child_bountes_total}</h3>
                    <h3 class="md:text-lg font-bold text-gray-900 dark:text-gray-100">{"Child Bounties"}</h3>
                </div>
                <ul class="tab tab__filters">
                    { for Filter::iter().map(|filter| {
                        html! {
                            <FilterItem filter={filter.clone()}
                                selected={state.filter.to_string() == filter.to_string()}
                                onclick={&onclick}
                            />
                        }
                    }) }
                </ul>
            </div>
        }
    } else {
        html! {}
    }
}

#[function_component(ChildBountiesTitle)]
pub fn child_bounties_title() -> Html {
    let state = use_context::<StateContext>().unwrap();

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        let child_bountes_total = child_bounties_raw
            .into_iter()
            .filter(|(_, cb)| state.filter.check(cb))
            .count();

        if child_bountes_total > 0 {
            return html! {
                <div class="flex flex-none justify-between items-center mb-4">
                    <div class="inline-flex items-center">
                        <h3 class="md:text-lg text-gray-900 dark:text-gray-100 me-1">{child_bountes_total}</h3>
                        <h3 class="md:text-lg font-bold text-gray-900 dark:text-gray-100 me-4">{"Child Bounties"}</h3>
                        <Spinner is_visible={state.network.is_fetching()} />
                    </div>

                    {
                        if state.filter.is_following() {
                            html!{ <ClaimButton /> }
                        } else {
                            html! {}
                        }
                    }

                </div>
            };
        }
    }
    html! {}
}

#[function_component(ChildBountiesStats)]
pub fn child_bounties_stats() -> Html {
    let state = use_context::<StateContext>().unwrap();
    let runtime = state.network.runtime.clone();

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        if let Some(block_number) = state.network.finalized_block_number {
            let amount_pending = child_bounties_raw
                .into_iter()
                .filter(|(_, cb)| state.filter.check(cb) && !cb.is_claimable(block_number))
                .map(|(_, cb)| cb.value)
                .sum::<u128>();

            let amount_claimable = child_bounties_raw
                .into_iter()
                .filter(|(_, cb)| state.filter.check(cb) && cb.is_claimable(block_number))
                .map(|(_, cb)| cb.value)
                .sum::<u128>();

            return html! {
                <div class="mb-4">
                    <hr/>
                    <div class="flex mx-2 my-2 justify-center items-center">
                        {
                            if !state.filter.is_claimable() {
                                html! {
                                    <div class="text-center me-8">
                                        <div class="text-xs font-light dark:text-gray-100">{"Total Pending"}</div>
                                        <div class="inline-flex">
                                            <span class="text-xs font-bold dark:text-gray-100 me-1">{amount_human(amount_pending, runtime.decimals().into())}</span>
                                            <span class="text-xs dark:text-gray-100">{runtime.unit()}</span>
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        <div class="text-center">
                            <div class="text-xs font-light dark:text-gray-100">{"Total Claimable"}</div>
                            <div class="inline-flex">
                                <span class="text-xs font-bold dark:text-gray-100 me-1">{amount_human(amount_claimable, runtime.decimals().into())}</span>
                                <span class="text-xs dark:text-gray-100">{runtime.unit()}</span>
                            </div>
                        </div>

                    </div>
                    <hr/>
                </div>
            };
        }
    }
    html! {}
}

#[function_component(ChildBountiesBody)]
pub fn child_bounties_body() -> Html {
    let state = use_context::<StateContext>().unwrap();
    let input_value = use_state(|| "".to_string());

    let oninput = {
        let input_value = input_value.clone();
        Callback::from(move |value| {
            input_value.set(value);
        })
    };

    let onclear = {
        let input_value = input_value.clone();
        Callback::from(move |_| {
            input_value.set("".to_string());
        })
    };

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        let child_bountes_total = child_bounties_raw
            .into_iter()
            .filter(|(_, cb)| state.filter.check(cb))
            .count();

        html! {
            <>
                {
                    if state.layout.is_onboarding {
                        html! {
                            <FilterInput value={(*input_value).clone()} placeholder="Filter by Child Bounty description"
                                oninput={&oninput} onclear={&onclear}/>
                        }
                    } else { html! {} }
                }

                {
                    if child_bountes_total > 0 {
                        html! {
                            <ul class="flex-col w-full space-y space-y-4 text-sm font-medium text-gray-500 dark:text-gray-400">
                                {
                                    for child_bounties_raw.into_iter()
                                        .filter(|(_, cb)| state.filter.check(cb) && cb.description.to_lowercase().contains(&(*input_value).to_lowercase()))
                                        .map(|(_, cb)|
                                    html! {
                                        <ChildBountyItem id={cb.id} is_action_hidden={!state.layout.is_onboarding} />
                                    })
                                }
                            </ul>
                        }
                    } else if state.network.is_fetching() {
                        html! {
                            <div class="flex flex-col flex-1 justify-center items-center">
                                <Spinner is_visible={true} />
                                <p class="mt-4 text-xs text-center">{"Searching for any open child bounties awarded to the accounts you follow. Hang tight..."}</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="flex flex-col flex-1 justify-center items-center">
                                <p class="mt-4 text-xs text-center">{"There are no open child bounties awarded to the accounts you follow."}</p>
                            </div>
                        }
                    }
                }
            </>
        }
    } else {
        html! {}
    }
}
