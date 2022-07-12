
use std::collections::HashMap;

use super::item::{Item, self};

pub enum MatchType{
    TotalMatch,
    PartialMatch,
    ZeroMatch
}

//TODO add support for variable length crafting recipes
#[derive(Clone)]
pub struct CraftingRecipe{
    Grid: Vec<Item>,
    Rows: u32,
    Cols: u32
}

impl CraftingRecipe{
    pub fn New(grid: Vec<u8>, rows: u32, cols: u32) -> Result<Self, String> {
        if rows != cols || rows <= 0 || cols <= 0 {
            return Err(format!("Error! Invalid crafting grid dimensions of {} rows and {} columns!", rows, cols));
        }

        let itemGrid: Vec<Item> = grid.into_iter().map(|val| Item { ID: val } ).collect();

        Ok (
            Self {
                Grid: itemGrid, //move assignment
                Rows: rows,
                Cols: cols,
            }
        )
    }

    pub fn MatchType(&self, grid: &Vec<Item>) -> MatchType {
        MatchType::TotalMatch
    }
}

impl PartialEq for CraftingRecipe{
    fn eq(&self, other: &Self) -> bool {
        self.Rows == other.Rows && self.Cols == other.Cols && ! self.Grid.iter().zip(other.Grid.iter()).any(|x| *x.0 != *x.1)
    }
}

impl Eq for CraftingRecipe {}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct CraftingRegistry{
    //Hashmap is used so we can easily query for a specific item's recipe without having to loop over all recipes 
    Recipes: HashMap<u8, CraftingRecipe>
}

impl CraftingRegistry{
    pub fn New() -> Self {
        Self { Recipes: HashMap::new() }
    }

    pub fn AddRecipe(&mut self, itemID: u8, recipe: CraftingRecipe){
        self.Recipes.insert(itemID, recipe);
    }

    pub fn DoesRecipeExistFor(&self, item: &Item) -> bool{
        self.Recipes.contains_key(&item.ID)
    }

    pub fn GetRecipeFor(&self, item: &Item) -> Option<&CraftingRecipe> {
        self.Recipes.get(&item.ID)
    }

    pub fn MatchRecipe(&self, recipe: &CraftingRecipe) -> Option<Item> {
        let result = self.Recipes.iter().find(|&x| x.1 == recipe);
        if let Some(val) = result {
            return Some(Item { ID: *val.0 });
        }
        None
    }
}