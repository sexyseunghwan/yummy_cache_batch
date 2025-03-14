use crate::common::*;


pub trait QueryService {}

#[derive(Debug, new)]
pub struct QueryServicePub;

impl QueryService for QueryServicePub {

}