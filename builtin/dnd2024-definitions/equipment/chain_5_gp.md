---
splat_schema: 2
splat_id: 16ecaf16-855f-5f92-956e-891647177473
splat_kind: item
splat_key: dnd2024:equipment/chain_5_gp
splat_name: Chain (5 GP)
splat_origin: dnd2024:reference/pages/096
splat_sources:
- SRD 5.2.1, pp. 96, https://www.dndbeyond.com/srd
---

# Chain (5 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 96 -->

As a Utilize action, you can wrap a Chain around an unwilling creature within 5 feet of yourself that has the Grappled, Incapacitated, or Restrained condition if you succeed on a DC 13 Strength (Athletics) check. If the creature’s legs are bound, the creature has the Restrained condition until it escapes. Escaping the Chain requires the creature to make a successful DC 18 Dexterity (Acrobatics) check as an action. Bursting the Chain requires a successful DC 20 Strength (Athletics) check as an action.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/chain_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_chain_5_gp ` |

### Parameters

#### Bind

<!-- splat:yaml path=splat_rule/parameters/bind -->

```yaml
action: utilize
target:
  range_ft: 5
  unwilling: true
  has_any_condition:
  - grappled
  - incapacitated
  - restrained
check:
  ability: strength
  skill: athletics
  dc: 13
bound_legs_condition: restrained
until: escape
```

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
economy: action
check:
  ability: dexterity
  skill: acrobatics
  dc: 18
```

#### Burst

<!-- splat:yaml path=splat_rule/parameters/burst -->

```yaml
economy: action
check:
  ability: strength
  skill: athletics
  dc: 20
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 10.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 5
> mass:
>   unit: lb
>   value: 10.0
> ```

