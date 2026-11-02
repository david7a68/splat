---
splat_schema: 1
splat_id: 91e85153-b0e2-5bcf-8629-5714061ba51c
splat_kind: rule_group
splat_key: dnd2024:species/dwarf
splat_name: Dwarf
splat_origin:
  key: dnd2024:reference/pages/084
splat_sources:
- title: SRD 5.2.1
  locator: pp. 84
  url: https://www.dndbeyond.com/srd
---

# Dwarf

## Source text

<!-- SRD page 84 -->

Creature Type: Humanoid\
Size: Medium (about 4–5 feet tall)\
Speed: 30 feet

As a Dwarf, you have these special traits.

Darkvision. You have Darkvision with a range of 120 feet.

Dwarven Resilience. You have Resistance to Poison damage. You also have Advantage on saving throws you make to avoid or end the Poisoned condition.

Dwarven Toughness. Your Hit Point maximum increases by 1, and it increases by 1 again whenever you gain a level.

Stonecunning. As a Bonus Action, you gain Tremorsense with a range of 60 feet for 10 minutes. You must be on a stone surface or touching a stone surface to use this Tremorsense. The stone can be natural or worked.

You can use this Bonus Action a number of times equal to your Proficiency Bonus, and you regain all expended uses when you finish a Long Rest.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` species ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:species/dwarf ` |
| ` implementation ` | ` dnd2024:species_dwarf ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` humanoid ` |
| ` size ` | ` medium ` |
| ` speed_feet ` | ` 30 ` |
| ` darkvision_feet ` | ` 120 ` |

#### Height feet

<!-- splat:values path=splat_rule/parameters/height_feet -->

| Field | Value |
| --- | --- |
| ` typical_minimum ` | ` 4 ` |
| ` typical_maximum ` | ` 5 ` |

#### Dwarven resilience

<!-- splat:yaml path=splat_rule/parameters/dwarven_resilience -->

```yaml
damage_resistance:
- poison
save_advantage:
  condition: poisoned
  purposes:
  - avoid
  - end
```

#### Dwarven toughness

<!-- splat:yaml path=splat_rule/parameters/dwarven_toughness -->

```yaml
hit_point_maximum_bonus:
  var: actor.character_level
recalculate_on_level_gain: true
```

#### Stonecunning

<!-- splat:yaml path=splat_rule/parameters/stonecunning -->

```yaml
action: bonus_action
resource:
  id: dwarf.stonecunning
  maximum:
    var: actor.proficiency_bonus
  recovery:
  - event: rest.long_completed
    amount: all
cost: 1
duration:
  minutes: 10
tremorsense_feet: 60
sense_requires:
  stone_surface_contact: true
  stone_types:
  - natural
  - worked
```
