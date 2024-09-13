/*
 * Represents Block object in Minecraft
 *
 * Authored by: Three rats in a trench coat.
 */
#![allow(non_snake_case, non_camel_case_types)]


pub struct block{
    x: i32,
    y: i32,
    z: i32,
    id: i32,
}
impl block {
    pub fn new(x: i32, y: i32, z: i32, id: i32) -> block {
        block{x: x, y: y, z: z, id: id}
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn get_z(&self) -> i32 {
        self.z
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
}

pub enum block_material {
    ACACIA_WOOD,
    ACACIA_WOOD_STAIRS,
    ACACIA_WOOD_PLANK,
    ACTIVATOR_RAIL,
    ALLIUM,
    ANVIL,
    BEACON,
    BED_BLOCK,
    BEADROCK,
    BIRCH_WOOD_STAIRS,
    BIRCH_WOOD_PLANK,
    BLACK_CARPET,
    BLACK_STAINED_CLAY,
    BLACK_STAINED_GLASS,
    BLACK_STAINED_GLASS_PANE,
    BLACK_WOOL,
    BLOCK_OF_COAL,
    BLUE_CARPET,
    BLUE_ORCHID,
    BLUE_STAINED_CLAY,
    BLUE_STAINED_GLASS,
    BLUE_STAINED_GLASS_PANE,
    BLUE_WOOL,
    BOOKSHELF,
    BRICK,
    BRICK_SLAB,
    BRICK_STAIRS,
    BROWN_CARPET,
    BROWN_MUSHROOM,
    BROWN_MUSHROOM_CAP,
    BROWN_STAINED_CLAY,
    BROWN_STAINED_GLASS,
    BROWN_STAINED_GLASS_PANE,
    BROWN_WOOL,
    CACTUS,
    CAKE_BLOCK,
    CARROT_BLOCK,
    CAULDRON,
    CHEST,
    CHISELED_QUARTZ_BLOCK,
    CHISELED_SANDSTONE,
    CHISELED_STONE_MONSTER_EGG,
    CLAY_BLOCK,
    COAL_ORE,
    COBBLESTONE,
    COBBLESTONE_MONSTER_EGG,
    COBBLESTONE_SLAB,
    COBBLESTONE_STAIRS,
    COBBLESTONE_WALL,
    COCOA_PLANT,
    COMMAND_BLOCK,
    CRACKED_STONE_BRICK,
    CRACKED_STONE_BRICK_MONSTER_EGG,
    CROPS,
    CYAN_CARPET,
    CYAN_STAINED_CLAY,
    CYAN_STAINED_GLASS,
    CYAN_STAINED_GLASS_PANE,
    CYAN_WOOL,
    DARK_OAK_WOOD,
    DARK_OAK_WOOD_STAIRS,
    DARK_OAK_WOOD_PLANK,
    DAYLIGHT_SENSOR,
    DEAD_SHRUB,
    DETECTOR_RAIL,
    DIAMOND_BLOCK,
    DIAMOND_ORE,
    DIRT,
    DOUBLE_BRICK_SLAB,
    DOUBLE_COBBLESTONE_SLAB,
    DOUBLE_NETHER_BRICK_SLAB,
    DOUBLE_OAK_WOOD_SLAB,
    DOUBLE_QUARTZ_SLAB,
    DOUBLE_SANDSTONE_SLAB,
    DOUBLE_STONE_BRICK_SLAB,
    DOUBLE_STONE_SLAB,
    DOUBLE_TALLGRASS,
    DOUBLE_WOODEN_SLAB,
    DRAGON_EGG,
    DROPPER,
    EMERALD_BLOCK,
    EMERALD_ORE,
    ENCHANTMENT_TABLE,
    END_PORTAL,
    END_PORTAL_FRAME,
    END_STONE,
    ENDER_CHEST,
    FARMLAND,
    FENCE_GATE,
    FIRE,
    FLOWER_POT_BRICK,
    FURNACE,
    GLASS,
    GLASS_PANE,
    GLOWING_REDSTONE_ORE,
    GLOWSTONE_BLOCK,
    GOLD_BLOCK,
    GOLD_ORE,
    GRASS,
    GRAVEL,
    GRAY_CARPET,
    GRAY_STAINED_CLAY,
    GRAY_STAINED_GLASS,
    GRAY_STAINED_GLASS_PANE,
    GRAY_WOOL,
    GREEN_CARPET,
    GREEN_STAINED_CLAY,
    GREEN_STAINED_GLASS,
    GREEN_STAINED_GLASS_PANE,
    GREEN_WOOL,
    HARDENED_CLAY,
    HAY_BALE,
    HOPPER,
    ICE,
    IRON_BARS,
    IRON_BLOCK,
    IRON_DOOR,
    IRON_ORE,
    JACK_O_LANTERN,
    JUNGLE_WOOD_STAIRS,
    JUNGLE_WOOD_PLANK,
    LADDER,
    LAPIS_BLOCK,
    LAPIS_ORE,
    LARGE_FERN,
    LAVA,
    LEAVES,
    LEAVES_BIRCH,
    LEAVES_JUNGLE,
    LEAVES_SPRUCE,
    LEVER,
    LIGHT_BLUE_CARPET,
    LIGHT_BLUE_STAINED_CLAY,
    LIGHT_BLUE_STAINED_GLASS,
    LIGHT_BLUE_STAINED_GLASS_PANE,
    LIGHT_BLUE_WOOL,
    LIGHT_GRAY_CARPET,
    LIGHT_GRAY_STAINED_CLAY,
    LIGHT_GRAY_STAINED_GLASS,
    LIGHT_GRAY_STAINED_GLASS_PANE,
    LIGHT_GRAY_WOOL,
    LILAC,
    LILY_PAD,
    LIME_CARPET,
    LIME_STAINED_CLAY,
    LIME_STAINED_GLASS,
    LIME_STAINED_GLASS_PANE,
    LIME_WOOL,
    LIVE_SHRUB,
    MAGENTA_CARPET,
    MAGENTA_STAINED_CLAY,
    MAGENTA_STAINED_GLASS,
    MAGENTA_STAINED_GLASS_PANE,
    MAGENTA_WOOL,
    MELON_BLOCK,
    MOB_HEAD,
    MONSTER_SPAWNER,
    MOSSY_COBBLESTONE_WALL,
    MOSSY_STONE,
    MOSSY_STONE_BRICK,
    MOSSY_STONE_BRICK_MONSTER_EGG,
    MYCELIUM,
    NETHER_BRICK,
    NETHER_BRICK_SLAB,
    NETHER_QUARTZ_ORE,
    NETHERRACK,
    NOTE_BLOCK,
    OBSIDIAN,
    ORANGE_CARPET,
    ORANGE_STAINED_CLAY,
    ORANGE_STAINED_GLASS,
    ORANGE_STAINED_GLASS_PANE,
    ORANGE_TULIP,
    ORANGE_WOOL,
    OXEYE_DAISY,
    PACKED_ICE,
    PEONY,
    PILLAR_QUARTZ_BLOCK,
    PINK_CARPET,
    PINK_STAINED_CLAY,
    PINK_STAINED_GLASS,
    PINK_STAINED_GLASS_PANE,
    PINK_TULIP,
    PINK_WOOL,
    PORTAL,
    POTATO_BLOCK,
    POWERED_RAIL,
    PUMPKIN,
    PURPLE_CARPET,
    PURPLE_STAINED_CLAY,
    PURPLE_STAINED_GLASS,
    PURPLE_STAINED_GLASS_PANE,
    PURPLE_WOOL,
    QUARTZ_BLOCK,
    QUARTS_SLAB,
    RED_CARPET,
    RED_MUSHROOM_CAP,
    RED_SAND,
    RED_STAINED_CLAY,
    RED_STAINED_GLASS,
    RED_STAINED_GLASS_PANE,
    RED_TULIP,
    RED_WOOL,
    REDSTONE_BLOCK,
    REDSTONE_COMPARATOR,
    REDSTONE_COMPARATOR_ACTIVE,
    REDSTONE_COMPARATOR_INACTIVE,
    REDSTONE_LAMP_OFF,
    REDSTONE_LAMP_ON,
    REDSTONE_ORE,
    REDSTONE_REPEATER,
    REDSTONE_REPEATER_ACTIVE,
    REDSTONE_REPEATER_INACTIVE,
    REDSTONE_TORCH_OFF,
    REDSTONE_TORCH_ON,
    ROSE_BUSH,
    SAND,
    SANDSTONE,
    SANDSTONE_SLAB,
    SANDSTONE_STAIRS,
    SIGN_POST,
    SMOOTH_SANDSTONE,
    SNOW,
    SOUL_SAND,
    SPONGE,
    SPRUCE_WOOD_STAIRS,
    SPRUCE_WOOD_PLANK,
    STATIONARY_LAVA,
    STATIONARY_WATER,
    STICKY_PISTON,
    STONE,
    STONE_BRICK_MONSTER_EGG,
    STONE_BRICK_SLAB,
    STONE_BRICK_STAIRS,
    STONE_MONSTER_EGG,
    STONE_PRESSURE_PLATE,
    STONE_SLAB,
    SUGAR_CANE_BLOCK,
    SUNFLOWER,
    TALL_DEAD_SHRUB,
    TALL_GRASS,
    TNT,
    TORCH,
    TRAP_DOOR,
    TRAPPED_CHEST,
    TRIPWIRE,
    TRIPWIRE_HOOK,
    VINE,
    WATER,
    WEB_BLOCK,
    WEIGHTED_PRESSURE_PLATE_HEAVY,
    WEIGHTED_PRESSURE_PLATE_LIGHT,
    WHITE_CARPET,
    WHITE_STAINED_CLAY,
    WHITE_STAINED_GLASS,
    WHITE_STAINED_GLASS_PANE,
    WHITE_TULIP,
    WHITE_WOOL,
    WOOD_BIRCH,
    WOOD_OAK,
    WOOD_SPRUCE,
    WOOD_STAIRS,
    WOODEN_BUTTON,
    WOODEN_PLANK,
    WOODEN_PRESSURE_PLATE,
    WOODEN_SLAB,
    WOODEN_SLAB_BLOCK,
    YELLOW_CARPET,
    YELLOW_STAINED_CLAY,
    YELLOW_STAINED_GLASS,
    YELLOW_STAINED_GLASS_PANE,
    YELLOW_WOOL,
}