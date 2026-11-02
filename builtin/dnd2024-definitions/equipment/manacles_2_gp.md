---
splat_schema: 1
splat_id: 20bc9590-5fc2-5f8e-ab11-5993354b875f
splat_kind: item
splat_key: dnd2024:equipment/manacles_2_gp
splat_name: Manacles (2 GP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98
  url: https://www.dndbeyond.com/srd
---

# Manacles (2 GP)

## Source text

<!-- SRD page 98 -->

As a Utilize action, you can use Manacles to bind an unwilling Small or Medium creature within 5 feet of yourself that has the Grappled, Incapacitated, or Restrained condition if you succeed on a DC 13 Dexterity (Sleight of Hand) check. While bound, a creature has Disadvantage on attack rolls, and the creature is Restrained if the Manacles are attached to a chain or hook that is fixed in place. Escaping the Manacles requires a successful DC 20 Dexterity (Sleight of Hand) check as an action. Bursting them requires a successful DC 25 Strength (Athletics) check as an action.

Each set of Manacles comes with a key. Without the key, a creature can use Thieves’ Tools to pick the Manacles’ lock with a successful DC 15 Dexterity (Sleight of Hand) check.

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 6.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/manacles_2_gp ` |
| ` implementation ` | ` dnd2024:equipment_manacles_2_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` includes_key ` | ` true ` |

#### Bind

<!-- splat:yaml path=splat_rule/parameters/bind -->

```yaml
action: utilize
target:
  range_ft: 5
  sizes:
  - small
  - medium
  unwilling: true
  has_any_condition:
  - grappled
  - incapacitated
  - restrained
check:
  ability: dexterity
  skill: sleight_of_hand
  dc: 13
while_bound:
  attack_roll_mode: disadvantage
  restrained_if: attached_to_fixed_chain_or_hook
```

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
economy: action
check:
  ability: dexterity
  skill: sleight_of_hand
  dc: 20
```

#### Burst

<!-- splat:yaml path=splat_rule/parameters/burst -->

```yaml
economy: action
check:
  ability: strength
  skill: athletics
  dc: 25
```

#### Pick

<!-- splat:yaml path=splat_rule/parameters/pick -->

```yaml
requires_item:
  key: dnd2024:equipment/other_tools_thieves_tools_25_gp
check:
  ability: dexterity
  skill: sleight_of_hand
  dc: 15
success: unlock
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 6.0 ` |
| ` unit ` | ` lb ` |
