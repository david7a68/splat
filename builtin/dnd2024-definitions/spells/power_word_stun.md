---
splat_schema: 1
splat_id: 64232bd1-5e1c-54a7-9975-06561b611acc
splat_kind: spell
splat_key: dnd2024:spells/power_word_stun
splat_name: Power Word Stun
splat_origin:
  key: dnd2024:reference/spell/power_word_stun
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154
  url: https://www.dndbeyond.com/srd
---

# Power Word Stun

## Source text

<!-- SRD page 154 -->

Level 8 Enchantment (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous

You overwhelm the mind of one creature you can see within range. If the target has 150 Hit Points or fewer, it has the Stunned condition. Otherwise, its Speed is 0 until the start of your next turn.

The Stunned target makes a Constitution saving throw at the end of each of its turns, ending the condition on itself on a success.

Original layout: [[../../srd-5.2.1/pages/154|page 154]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 8 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
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
| ` mechanic ` | ` dnd2024:spells/power_word_stun ` |
| ` implementation ` | ` dnd2024:spell_power_word_stun ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |

#### Branches

<!-- splat:table
path: splat_rule/parameters/branches
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Hp lte:
    path: hp_lte
    type: integer
    optional: true
  Condition:
    path: condition
    type: symbol
    optional: true
  Repeat save event:
    path: repeat_save/event
    type: string
    optional: true
  Repeat save ability:
    path: repeat_save/ability
    type: symbol
    optional: true
  Repeat save success:
    path: repeat_save/success
    type: symbol
    optional: true
  Hp gt:
    path: hp_gt
    type: integer
    optional: true
  Speed:
    path: speed
    type: integer
    optional: true
  Until:
    path: until
    type: string
    optional: true
-->

| Key | Hp lte | Condition | Repeat save event | Repeat save ability | Repeat save success | Hp gt | Speed | Until |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 150 ` | ` Stunned ` | ` turn.end ` | ` Constitution ` | ` End stunned ` |  |  |  |
| ` row_2 ` |  |  |  |  |  | ` 150 ` | ` 0 ` | ` caster.next_turn.start ` |
