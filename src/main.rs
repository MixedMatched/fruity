use rcas::{
    expression::expression_tree::Expression,
    modifier::default::{approximator, evaluator, simplifier},
};
use sycamore::prelude::*;

#[component(inline_props)]
fn EvaluatedView<G: Html>(cx: Scope, expr: Expression) -> View<G> {
    let simpl = simplifier();
    let eval = evaluator();
    let approx = approximator();

    let (expr_eval, expr_approx) = expr.evaluate_im::<_,_,_,100>(&approx, &eval, &simpl);

    if let Some(approx) = expr_approx {
        view! {cx, 
            p {
                (expr_eval) 
                " ≈ "
                (approx)
            }
        }
    } else {
        view! {cx, 
            p {
                (expr_eval)
            }
        }
    }
}

fn main() {
    sycamore::render(|cx| {
        let expr_state = create_signal(cx, String::new());

        view! { cx,
            div {
                h1 { "Fruity" }
                p { "Enter an expression:" }
                input(bind:value=expr_state)
                (if expr_state.get().len() != 0 {
                    if let Ok(expr) = (expr_state.get().as_str()).parse::<Expression>() {
                        view! {cx, EvaluatedView(expr=expr)}
                    } else {
                        view! {cx, p { "Invalid expression" }}
                    }
                } else {
                    view! {cx, p { "No expression entered" }}
                })
            }
        }
    });
}
