---
splat_schema: 1
splat_id: 40c57379-6f88-5f05-9479-5208a5cd0e9b
splat_kind: item
splat_key: dnd2024:items/dancing_sword
splat_name: Dancing Sword
splat_origin:
  key: dnd2024:reference/magic_item/dancing_sword
splat_sources:
- title: SRD 5.2.1
  locator: pp. 217
  url: https://www.dndbeyond.com/srd
---

# Dancing Sword

## Source text

<!-- SRD page 217 -->

Weapon (Greatsword, Longsword, Rapier, Scimitar, or\
Shortsword), Very Rare (Requires Attunement)\
You can take a Bonus Action to toss this magic\
weapon into the air. When you do so, the weapon\
begins to hover, flies up to 30 feet, and attacks one\
creature of your choice within 5 feet of itself. The\
weapon uses your attack roll and adds your ability\
modifier to damage rolls.\
While the weapon hovers, you can take a Bonus\
Action to cause it to fly up to 30 feet to another spot\
within 30 feet of you. As part of the same Bonus Ac-\
tion, you can cause the weapon to attack one crea-\
ture within 5 feet of the weapon.\
After the hovering weapon attacks for the fourth\
time, it flies back to you and tries to return to your\
hand. If you have no hand free, the weapon falls to\
the ground in your space. If the weapon has no un-\
obstructed path to you, it moves as close to you as\
it can and then falls to the ground. It also ceases to\
hover if you grasp it or are more than 30 feet away\
from it.\

Original layout: [[../../srd-5.2.1/pages/217|page 217]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/dancing_sword ` |
| ` implementation ` | ` dnd2024:item_dancing_sword ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` after_attack_count ` | ` 4 ` |
| ` hover_ends_on ` | ` ["grasped", "distance_from_actor_over_30_ft"] ` |
| ` attunement_required ` | ` true ` |

#### Activate

<!-- splat:yaml path=splat_rule/parameters/activate -->

```yaml
action: bonus_action
toss: true
fly_ft: 30
attack:
  target_count: 1
  target_range_from_weapon_ft: 5
  attack_roll: actor.weapon_attack
  damage_modifier: actor.weapon_ability_modifier
```

#### Hover command

<!-- splat:yaml path=splat_rule/parameters/hover_command -->

```yaml
action: bonus_action
move_up_to_ft: 30
destination_range_from_actor_ft: 30
optional_attack:
  target_count: 1
  target_range_from_weapon_ft: 5
  attack_roll: actor.weapon_attack
  damage_modifier: actor.weapon_ability_modifier
```

#### Return behavior

<!-- splat:values path=splat_rule/parameters/return_behavior -->

| Field | Value |
| --- | --- |
| ` to ` | ` actor_hand ` |
| ` no_free_hand ` | ` drop_in_actor_space ` |
| ` obstructed_path ` | ` move_closest_then_drop ` |
