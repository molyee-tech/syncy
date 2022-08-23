use common::Link;

pub struct Id([u8; 32]);

pub struct User {
	id: Id,
	icon: Link,
	name: String,
}