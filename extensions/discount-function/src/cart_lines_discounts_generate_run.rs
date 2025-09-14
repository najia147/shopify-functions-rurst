use crate::schema::CartLineTarget;
use crate::schema::CartLinesDiscountsGenerateRunResult;
use crate::schema::CartOperation;
use crate::schema::DiscountClass;
use crate::schema::OrderDiscountCandidate;
use crate::schema::OrderDiscountCandidateTarget;
use crate::schema::OrderDiscountCandidateValue;
use crate::schema::OrderDiscountSelectionStrategy;
use crate::schema::OrderDiscountsAddOperation;
use crate::schema::OrderSubtotalTarget;
use crate::schema::Percentage;
use crate::schema::ProductDiscountCandidate;
use crate::schema::ProductDiscountCandidateTarget;
use crate::schema::ProductDiscountCandidateValue;
use crate::schema::ProductDiscountSelectionStrategy;
use crate::schema::ProductDiscountsAddOperation;

use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn cart_lines_discounts_generate_run(
    input: schema::cart_lines_discounts_generate_run::Input,
) -> Result<schema::CartLinesDiscountsGenerateRunResult> {
    let has_product = input
        .discount()
        .discount_classes()
        .contains(&schema::DiscountClass::Product);

    if !has_product {
        return Ok(schema::CartLinesDiscountsGenerateRunResult { operations: vec![] });
    }

    let mut candidates: Vec<schema::ProductDiscountCandidate> = vec![];

    // Check each cart line
   for line in input.cart().lines() {
    if *line.quantity() >= 2 {
        candidates.push(schema::ProductDiscountCandidate {
            targets: vec![schema::ProductDiscountCandidateTarget::CartLine(
                schema::CartLineTarget {
                    id: line.id().clone(),
                    quantity: None,
                },
            )],
            message: Some("30% off when you buy 2+".to_string()),
            value: schema::ProductDiscountCandidateValue::Percentage(schema::Percentage {
                value: Decimal(30.0),
            }),
            associated_discount_code: None,
        });
    }
}


    let mut operations = vec![];

    if !candidates.is_empty() {
        operations.push(schema::CartOperation::ProductDiscountsAdd(
            schema::ProductDiscountsAddOperation {
                selection_strategy: schema::ProductDiscountSelectionStrategy::First,
                candidates,
            },
        ));
    }

    Ok(schema::CartLinesDiscountsGenerateRunResult { operations })
}
