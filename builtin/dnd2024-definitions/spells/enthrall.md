---
splat_schema: 1
splat_id: c2287311-87b0-59ab-a189-62d6d78e671a
splat_kind: spell
splat_key: dnd2024:spells/enthrall
splat_name: Enthrall
splat_origin:
  key: dnd2024:reference/spell/enthrall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 128
  url: https://www.dndbeyond.com/srd
---

# Enthrall

## Source text

<!-- SRD page 128 -->

Level 2 Enchantment (Bard, Warlock)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

You weave a distracting string of words, causing creatures of your choice that you can see within range to make a Wisdom saving throw. Any creature you or your companions are fighting automatically succeeds on this save. On a failed save, a target has a −10 penalty to Wisdom (Perception) checks and Passive Perception until the spell ends.

Original layout: [[../../srd-5.2.1/pages/128|page 128]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "warlock"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/enthrall ` |
| ` implementation ` | ` dnd2024:spell_enthrall ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 60 ` |
| ` chooser ` | ` caster ` |
| ` count ` | ` any ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` automatic_success_if ` | ` caster_or_companions_fighting_target ` |

#### Failure

##### Modifiers

<!-- splat:table
path: splat_rule/parameters/failure/modifiers
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Selector:
    path: selector
    type: symbol
  Add:
    path: add
    type: integer
-->

| Key | Selector | Add |
| --- | --- | --- |
| ` row_1 ` | ` Wisdom perception checks ` | ` -10 ` |
| ` row_2 ` | ` Passive perception ` | ` -10 ` |
