//! Useful tools for managing profile types.
use std::ops::Deref;

use super::data::ProfilePropertyCode;
use crate::error::Error;
use echonet_lite::{EchonetObject, Properties};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct InstanceList(Vec<EchonetObject>);

impl InstanceList {
    /// Converts from a list of properties into a list of instances, erroring in case
    /// the property type is not available *or* malformed.
    pub fn from(properties: Properties) -> Result<Self, Error> {
        let prop = properties
            .iter()
            .find(|&p| (ProfilePropertyCode::SelfNodeInstanceListS as u8).eq(&p.epc))
            .ok_or_else(|| Error::UnsetProperty("InstanceList".to_owned()))?;

        let mut instances: Vec<EchonetObject> = Vec::new();

        let mut it = prop.edt.iter();

        let count = it.next().map(|&v| v as usize).unwrap_or(0);

        while let Some((a, b, c)) = it.next_tuple() {
            instances.push([*a, *b, *c].into());
        }

        if !count.eq(&instances.len()) {
            Err(Error::MalformedProperty(format!(
                "property count doesn't match: {count} != {}",
                instances.len()
            )))?;
        }

        Ok(InstanceList(instances))
    }
}

impl IntoIterator for InstanceList {
    type Item = EchonetObject;
    type IntoIter = <Vec<EchonetObject> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for InstanceList {
    type Target = Vec<EchonetObject>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
