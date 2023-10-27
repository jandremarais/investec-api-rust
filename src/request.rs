use serde::Serialize;

use crate::Error;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutliPaymentRequest {
    pub payment_list: Vec<Payment>,
}

impl MutliPaymentRequest {
    pub fn new(payment_list: Vec<Payment>) -> Self {
        Self { payment_list }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    beneficiary_id: String,
    amount: String,
    my_reference: String,
    their_reference: String,
    authoriser_a_id: Option<String>,
    authoriser_b_id: Option<String>,
    auth_period_id: Option<String>,
    faster_payment: Option<bool>,
}

pub struct PaymentBuilder {
    beneficiary_id: String,
    amount: Option<String>,
    my_reference: Option<String>,
    their_reference: Option<String>,
    authoriser_a_id: Option<String>,
    authoriser_b_id: Option<String>,
    auth_period_id: Option<String>,
    faster_payment: Option<bool>,
}

impl Payment {
    pub fn to(beneficiary_id: impl Into<String>) -> PaymentBuilder {
        PaymentBuilder {
            beneficiary_id: beneficiary_id.into(),
            amount: None,
            my_reference: None,
            their_reference: None,
            authoriser_a_id: None,
            authoriser_b_id: None,
            auth_period_id: None,
            faster_payment: None,
        }
    }
}

impl PaymentBuilder {
    pub fn build(self) -> Result<Payment, Error> {
        let beneficiary_id = self.beneficiary_id;
        let amount = self.amount.ok_or(Error::PaymentFieldUndefined {
            field: "amount".to_string(),
        })?;
        let my_reference = self.my_reference.ok_or(Error::PaymentFieldUndefined {
            field: "my_reference".to_string(),
        })?;
        let their_reference = self.their_reference.ok_or(Error::PaymentFieldUndefined {
            field: "their_reference".to_string(),
        })?;

        let payment = Payment {
            beneficiary_id,
            amount,
            my_reference,
            their_reference,
            authoriser_a_id: self.authoriser_a_id,
            authoriser_b_id: self.authoriser_b_id,
            auth_period_id: self.auth_period_id,
            faster_payment: self.faster_payment,
        };
        Ok(payment)
    }

    pub fn amount(mut self, amount: f32) -> Self {
        self.amount = Some(amount.to_string());
        self
    }
    pub fn my_reference(mut self, refernce: impl Into<String>) -> Self {
        self.my_reference = Some(refernce.into());
        self
    }
    pub fn their_reference(mut self, refernce: impl Into<String>) -> Self {
        self.their_reference = Some(refernce.into());
        self
    }
    pub fn authoriser_a_id(mut self, id: impl Into<String>) -> Self {
        self.authoriser_a_id = Some(id.into());
        self
    }
    pub fn authoriser_b_id(mut self, id: impl Into<String>) -> Self {
        self.authoriser_b_id = Some(id.into());
        self
    }
    pub fn auth_period_id(mut self, id: impl Into<String>) -> Self {
        self.auth_period_id = Some(id.into());
        self
    }
    pub fn faster_payment(mut self) -> Self {
        self.faster_payment = Some(true);
        self
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultiTransferRequest {
    pub transfer_list: Vec<TransferRequest>,
    pub profile_id: Option<String>,
}

impl MultiTransferRequest {
    pub fn new(transfer_list: Vec<TransferRequest>, profile_id: impl Into<Option<String>>) -> Self {
        Self {
            transfer_list,
            profile_id: profile_id.into(),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    pub beneficiary_account_id: String,
    pub amount: String,
    pub my_reference: String,
    pub their_reference: String,
}

pub struct TransferBuilder {
    pub beneficiary_account_id: Option<String>,
    pub amount: Option<f32>,
    pub my_reference: Option<String>,
    pub their_reference: Option<String>,
}

impl TransferBuilder {
    pub fn new() -> Self {
        Self {
            beneficiary_account_id: None,
            amount: None,
            my_reference: None,
            their_reference: None,
        }
    }

    pub fn build(self) -> Result<TransferRequest, Error> {
        let beneficiary_account_id =
            self.beneficiary_account_id
                .ok_or(Error::TransferRequestFieldUndefined {
                    field: "beneficiary_acocunt_id".to_string(),
                })?;
        let amount = self
            .amount
            .ok_or(Error::TransferRequestFieldUndefined {
                field: "amount".to_string(),
            })?
            .to_string();
        let my_reference = self
            .my_reference
            .ok_or(Error::TransferRequestFieldUndefined {
                field: "my_reference".to_string(),
            })?;
        let their_reference = self
            .their_reference
            .ok_or(Error::TransferRequestFieldUndefined {
                field: "their_reference".to_string(),
            })?;
        let req = TransferRequest {
            beneficiary_account_id,
            amount,
            my_reference,
            their_reference,
        };
        Ok(req)
    }

    pub fn beneficiary_account_id(mut self, account_id: &str) -> Self {
        self.beneficiary_account_id = Some(account_id.to_string());
        self
    }

    pub fn amount(mut self, amount: f32) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn my_reference(mut self, reference: &str) -> Self {
        self.my_reference = Some(reference.to_string());
        self
    }

    pub fn their_reference(mut self, reference: &str) -> Self {
        self.their_reference = Some(reference.to_string());
        self
    }
}
