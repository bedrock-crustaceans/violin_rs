use crate::vio::{Buildable, ColorCode, Identifier, RangeDescriptor};
use serde::{Serialize};
use item_component_macros::item_component;
use crate::block::utils::BlockDestroySpeed;
use crate::item::utils::{DurabilityThreshold, EnchantableSlot, ItemRarity, ItemRepairEntry, ItemTextureDescriptor};

pub trait ItemComponent {
    fn serialize(&self) -> String;
}

// * ItemDamageComponent

item_component! {
    name = Damage for "minecraft:damage";
    value has i32 for "value" with "public" "optional";
}

// * ItemDisplayNameComponent

item_component! {
    name = DisplayName for "minecraft:display_name";
    value has String for "value" with "public" "into";
}

// * ItemIconComponent

item_component! {
    name = Icon for "minecraft:icon";
    textures has ItemTextureDescriptor for "textures" with "public";
}

// * ItemFuelComponent

item_component! {
    name = Fuel for "minecraft:fuel";
    duration has i32 for "duration" with "public";
}

// * ItemHandEquippedComponent

item_component! {
    name = HandEquipped for "minecraft:hand_equipped";
    value has bool for "value" with "public";
}

// * ItemAllowOffHandComponent

item_component! {
    name = AllowOffHand for "minecraft:allow_off_hand";
    value has bool for "value" with "public";
}

// * ItemMaxStackSizeComponent

item_component! {
    name = MaxStackSize for "minecraft:max_stack_size";
    value has i32 for "value" with "public";
}

// * Durability

item_component! {
    name = Durabilty for "minecraft:durability";
    damage_chance has RangeDescriptor<i32> for "damage_chance" with "public";
    max_durability has i32 for "max_durability" with "public";
}

// * ItemArmorComponent

item_component! {
    name = Armor for "minecraft:armor";
    protection has i32 for "protection" with "public";
}

// * ItemCreativeCategoryComponent

item_component! {
    name = CreativeCategory for "minecraft:creative_category";
    parent has String for "parent" with "public" "into";
}

// * ItemRepairableComponent

item_component! {
    name = Repairable for "minecraft:repairable";
    repair_items has Vec<ItemRepairEntry> for "repair_items" with "public";
}

// * ItemCustomComponents

item_component! {
    name = CustomComponents for "minecraft:custom_components" with "transparency";
    components has Vec<Identifier> for "minecraft:custom_components" with "public";
}

// * BundleInteraction

item_component! {
    name = BundleInteraction for "minecraft:bundle_interaction";
    viewable_slots has u8 for "num_viewable_slots" with "public";
}

// * CanDestroyInCreative

item_component! {
    name = CanDestroyInCreative for "minecraft:can_destroy_in_creative";
    value has bool for "value" with "public";
}

// * Cooldown

item_component! {
    name = Cooldown for "minecraft:cooldown";
    category has String for "category" with "public";
    duration has f64 for "duration" with "public";
}

// * DamageAbsorption

item_component! {
    name = DamageAbsorption for "minecraft:damage_absorption";
    absorbable_causes has Vec<String> for "absorbable_causes" with "public";
}

// * Digger

item_component! {
    name = Digger for "minecraft:digger";
    use_efficiency has bool for "use_efficiency" with "public";
    destroy_speeds has Vec<BlockDestroySpeed> for "destroy_speeds" with "public";
}

// * Enchantable

item_component! {
    name = Enchantable for "minecraft:enchantable";
    value has u8 for "value" with "public";
    slot has EnchantableSlot for "slot" with "public";
}

// * EntityPlacer

item_component! {
    name = EntityPlacer for "minecraft:entity_placer";
    entity has Identifier for "entity" with "public";
    dispense_on has Vec<Identifier> for "dispense_on" with "public";
    use_on has Vec<Identifier> for "use_on" with "public";
}

// * Glint

item_component! {
    name = Glint for "minecraft:glint";
    value has bool for "value" with "public";
}

// * HoverTextColor

item_component! {
    name = HoverTextColor for "minecraft:hover_text_color" with "transparency";
    color has ColorCode for "minecraft:hover_text_color" with "public";
}

// * DurabilitySensor

item_component! {
    name = DurabilitySensor for "minecraft:durability_sensor";
    durability_thresholds has Vec<DurabilityThreshold> for "durability_thresholds" with "public";
}

// * Dyeable

item_component! {
    name = Dyeable for "minecraft:dyeable";
    default_color has String for "default_color" with "public" "into";
}

// * InteractButton

item_component! {
    name = InteractButton for "minecraft:interact_button" with "transparency";
    value has String for "minecraft:interact_button" with "public" "into";
}

// * LiquidClipped

item_component! {
    name = LiquidClipped for "minecraft:liquid_clipped";
    value has bool for "value" with "public";
}

// * Projectile

item_component! {
    name = Projectile for "minecraft:projectile";
    minimum_critical_power has f64 for "minimum_critical_power" with "public";
    projectile_entity has Identifier for "projectile_entity" with "public";
}

// * Rarity

item_component! {
    name = Rarity for "minecraft:rarity" with "transparency";
    rarity has ItemRarity for "minecraft:rarity" with "public";
}
