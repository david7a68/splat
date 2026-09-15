---
splat_schema: 1
splat_id: 7e889f70-9118-5d06-ae0c-cbd5d05419de
splat_kind: item
splat_key: dnd2024:items/mace_of_disruption
splat_name: Mace of Disruption
splat_origin:
  key: dnd2024:reference/magic_item/mace_of_disruption
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229
  url: https://www.dndbeyond.com/srd
---

# Mace of Disruption

## Source text

<!-- SRD page 229 -->

Weapon (Mace), Rare (Requires Attunement)\
When you hit a Fiend or an Undead with this magic\
weapon, that creature takes an extra 2d6 Radiant\
damage. If the target has 25 Hit Points or fewer\
after taking this damage, it must succeed on a DC\
15 Wisdom saving throw or be destroyed. On a suc-\
cessful save, the creature has the Frightened condi-\
tion until the end of your next turn.\
Light. While you hold this weapon, it sheds Bright\
Light in a 20-foot radius and Dim Light for an addi-\
tional 20 feet.\

Original layout: [[../../srd-5.2.1/pages/229|page 229]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/mace_of_disruption ` |
| ` implementation ` | ` dnd2024:item_mace_of_disruption ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attunement_required ` | ` true ` |

#### On hit

<!-- splat:yaml path=splat_rule/parameters/on_hit -->

```yaml
target_types:
- fiend
- undead
damage:
  amount:
    roll:
      count: 2
      sides: 6
  type: radiant
after_damage_hp_at_most: 25
save:
  ability: wisdom
  dc: 15
  failure:
    destroy_target: true
  success:
    condition: frightened
    duration_end: user.next_turn_end
```

#### While held

<!-- splat:yaml path=splat_rule/parameters/while_held -->

```yaml
light:
  bright_radius_ft: 20
  additional_dim_ft: 20
```
