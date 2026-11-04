---
splat_schema: 2
splat_id: bd3f526f-3cbb-5518-9533-1c3215a5fe64
splat_kind: item
splat_key: dnd2024:items/rope_of_climbing
splat_name: Rope of Climbing
splat_origin: dnd2024:reference/magic_item/rope_of_climbing
splat_sources:
- SRD 5.2.1, pp. 242, https://www.dndbeyond.com/srd
---

# Rope of Climbing


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 242 -->

Wondrous Item, Uncommon

This 60-foot length of rope can hold up to 3,000 pounds. While holding one end of the rope, you can take a Magic action to command the other end of the rope to animate and move toward a destination you choose, up to the rope’s length away from you. That end moves 10 feet on your turn when you first command it and 10 feet at the start of each of your subsequent turns until reaching its destination or until you tell it to stop. You can also tell the rope to fasten itself securely to an object or to unfasten itself, to knot or unknot itself, or to coil itself for carrying.

If you tell the rope to knot, large knots appear at 1-foot intervals along the rope. While knotted, the rope shortens to a 50-foot length and grants Advantage on ability checks made to climb using the rope.

The rope has AC 20, HP 20, and Immunity to Poison and Psychic damage. It regains 1 Hit Point every 5 minutes as long as it has at least 1 Hit Point. If the rope drops to 0 Hit Points, it is destroyed.

Original layout: [[../../srd-5.2.1/pages/242|page 242]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/rope_of_climbing ` |
| ` implementation ` | ` dnd2024:item_rope_of_climbing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` length_ft ` | ` 60 ` |
| ` weight_capacity_lb ` | ` 3000 ` |
| ` ac ` | ` 20 ` |
| ` hp ` | ` 20 ` |
| ` damage_immunities ` | ` ["poison", "psychic"] ` |
| ` zero_hp ` | ` destroyed ` |
| ` commands ` | ` ["fasten_to_object", "unfasten", "knot", "unknot", "coil_for_carrying"] ` |

#### Regeneration

<!-- splat:yaml path=splat_rule/parameters/regeneration -->

```yaml
amount: 1
interval:
  amount: 5
  unit: minute
minimum_current_hp: 1
```

#### Animate

<!-- splat:values path=splat_rule/parameters/animate -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` one_end_held ` |
| ` action ` | ` magic ` |
| ` destination_maximum_distance ` | ` rope_length ` |
| ` movement_ft_on_activation ` | ` 10 ` |
| ` movement_ft_at_user_turn_start ` | ` 10 ` |
| ` ends_on ` | ` ["destination_reached", "stop_command"] ` |

#### Knotted

<!-- splat:values path=splat_rule/parameters/knotted -->

| Field | Value |
| --- | --- |
| ` knot_interval_ft ` | ` 1 ` |
| ` length_ft ` | ` 50 ` |
| ` ability_check_advantage ` | ` climb_using_rope ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
