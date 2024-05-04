use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use modql::filter::{FilterNodes, OpValsString, OpValsValue};
use modql::filter::{ListOptions, OpValsInt64};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

// region:    --- Project Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Item {
	pub id: i64,

	pub owner_id: i64,
	pub name: String,
	pub origin: String,
	pub grapes: String,
	pub good_with: String,
	pub acid: i8,
	pub alcohol: i8,
	pub body: i8,
	pub tannin: i8,

	// -- Timestamps
	//    (creator and last modified user_id/time)
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct ItemForCreate {
	pub name: String,
}

#[derive(Fields, Deserialize)]
pub struct ItemForUpdate {
	pub name: Option<String>,
	pub owner_id: Option<i64>,
}

#[derive(Fields)]
struct ItemForCreateInner {
	pub name: String,
	pub owner_id: i64,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct ItemFilter {
	id: Option<OpValsInt64>,
	name: Option<OpValsString>,

	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}
// endregion: --- Project Types

// region:    --- ProjectBmc
pub struct ItemBmc;

impl DbBmc for ItemBmc {
	const TABLE: &'static str = "item";
}

impl ItemBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		item_c: ItemForCreate,
	) -> Result<i64> {
		let item_c = ItemForCreateInner {
			name: item_c.name,
			owner_id: ctx.user_id(),
		};
		base::create::<Self, _>(ctx, mm, item_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Item> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<ItemFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Item>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		item_u: ItemForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, item_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
// endregion: --- ProjectBmc
