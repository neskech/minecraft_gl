
use std::collections::HashMap;
use super::item::ItemID;

pub enum MatchType{
    TotalMatch,
    PartialMatch(u32),
    ZeroMatch
}

//TODO add support for variable length crafting recipes
#[derive(Clone)]
pub struct CraftingRecipe{
    Grid: Vec<ItemID>,
    Rows: u32,
    Cols: u32
}

impl CraftingRecipe{
    pub fn New(grid: Vec<u8>, rows: u32, cols: u32) -> Result<Self, String> {
        if rows != cols || rows <= 0 || cols <= 0 {
            return Err(format!("Error! Invalid crafting grid dimensions of {} rows and {} columns!", rows, cols));
        }

        let itemGrid: Vec<ItemID> = grid.into_iter().map(|val| ItemID::New(val) ).collect();

        Ok (
            Self {
                Grid: itemGrid, //move assignment
                Rows: rows,
                Cols: cols,
            }
        )
    }

    pub fn MatchType(&self, other: &CraftingRecipe) -> MatchType {
        if self.Rows != other.Rows || self.Cols != other.Cols { return MatchType::ZeroMatch; }
        let mut selfSet: HashMap<ItemID, u16> = HashMap::new();
        let mut otherSet: HashMap<ItemID, u16> = HashMap::new();

        let mut matches = true;
        let mut matchCount = 0;
        for r in 0..self.Rows {
            for c in 0..self.Cols {
                let idx = r * self.Cols + c;
                let item1 = self.Grid[idx as usize];
                let item2 = other.Grid[idx as usize];

                if !selfSet.contains_key(&item1) {
                    selfSet.insert(item1, 1);
                }
                else {
                    *selfSet.get_mut(&item1).unwrap() += 1;
                }

                if !otherSet.contains_key(&item2) {
                    otherSet.insert(item2, 1);
                }
                else {
                    *otherSet.get_mut(&item2).unwrap() += 1;
                }

                if item1 != item2 {
                    matches = false;
                }
                else{
                    //at the end of the loop, this is incremented by a total of 2
                    //Makes it more favorable to have matches AND matches at the same index
                    matchCount += 1; 
                }

                if selfSet.contains_key(&item2) {
                    matchCount += 1;

                    *selfSet.get_mut(&item2).unwrap() -= 1;
                    if selfSet[&item2] == 0 { selfSet.remove(&item2); }
                    *otherSet.get_mut(&item2).unwrap() -= 1;
                    if otherSet[&item2] == 0 { otherSet.remove(&item2); }
                }
                else if otherSet.contains_key(&item1) {
                    matchCount += 1;

                    *selfSet.get_mut(&item1).unwrap() -= 1;
                    if selfSet[&item1] == 0 { selfSet.remove(&item1); }
                    *otherSet.get_mut(&item1).unwrap() -= 1;
                    if otherSet[&item1] == 0 { otherSet.remove(&item1); }
                }
                
            }
        }

        if matches {
            return MatchType::TotalMatch;
        }
        MatchType::PartialMatch(matchCount)

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

    pub fn DoesRecipeExistFor(&self, item: &ItemID) -> bool{
        self.Recipes.contains_key(&item.ID)
    }

    pub fn GetRecipeFor(&self, item: &ItemID) -> Option<&CraftingRecipe> {
        self.Recipes.get(&item.ID)
    }

    pub fn MatchRecipe(&self, recipe: &CraftingRecipe) -> Option<ItemID> {
        let result = self.Recipes.iter().find(|&x| x.1 == recipe);
        if let Some(val) = result {
            return Some(ItemID::New(*val.0));
        }
        None
    }
}