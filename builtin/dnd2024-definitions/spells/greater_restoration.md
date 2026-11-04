---
splat_schema: 2
splat_id: 55c81461-36e7-5f03-97ce-b861b2cdc156
splat_kind: spell
splat_key: dnd2024:spells/greater_restoration
splat_name: Greater Restoration
splat_origin: dnd2024:reference/spell/greater_restoration
splat_sources:
- SRD 5.2.1, pp. 137, https://www.dndbeyond.com/srd
---

# Greater Restoration


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Abjuration |
| Classes | Bard, Cleric, Druid, Paladin, Ranger |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (diamond dust worth 100+ GP, which the spell consumes) |
| Duration | Instantaneous |


## Source text

<!-- SRD page 137 -->

Level 5 Abjuration (Bard, Cleric, Druid, Paladin, Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (diamond dust worth 100+ GP, which the spell consumes)\
Duration: Instantaneous

You touch a creature and magically remove one of the following effects from it:

•  1 Exhaustion level

•  The Charmed or Petrified condition

•  A curse, including the target’s Attunement to a cursed magic item

•  Any reduction to one of the target’s ability scores

•  Any reduction to the target’s Hit Point maximum

Original layout: [[../../srd-5.2.1/pages/137|page 137]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/greater_restoration ` |
| ` implementation ` | ` dnd2024:spell_greater_restoration ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` remove_choice_count ` | ` 1 ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` touch ` | ` true ` |
| ` count ` | ` 1 ` |

#### Choices

<!-- splat:table
path: splat_rule/parameters/choices
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
columns:
  Key:
    type: key
  Kind:
    path: kind
    type: symbol
  Levels:
    path: levels
    type: integer
    optional: true
  Choices:
    path: choices
    type: yaml
    optional: true
  Include:
    path: include
    type: symbol
    optional: true
  Ability count:
    path: ability_count
    type: integer
    optional: true
  Reduction:
    path: reduction
    type: symbol
    optional: true
-->

| Key | Kind | Levels | Choices | Include | Ability count | Reduction |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Exhaustion ` | ` 1 ` |  |  |  |  |
| ` row_2 ` | ` Condition ` |  | ` ["charmed", "petrified"] ` |  |  |  |
| ` row_3 ` | ` Curse ` |  |  | ` Cursed item attunement ` |  |  |
| ` row_4 ` | ` Ability score reduction ` |  |  |  | ` 1 ` | ` Any ` |
| ` row_5 ` | ` Hp maximum reduction ` |  |  |  |  | ` Any ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> range:
>   mode: touch
> ```

