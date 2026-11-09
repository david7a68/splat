---
splat_schema: 2
splat_id: d8025a32-bf23-5965-b773-6e93bdf2f343
splat_kind: spell
splat_key: dnd2024:spells/bestow_curse
splat_name: Bestow Curse
splat_origin: dnd2024:reference/spell/bestow_curse
splat_sources:
- SRD 5.2.1, pp. 112, 113, https://www.dndbeyond.com/srd
---

# Bestow Curse


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Necromancy |
| Classes | Bard, Cleric, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 112 -->

Level 3 Necromancy (Bard, Cleric, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Concentration, up to 1 minute

You touch a creature, which must succeed on a Wisdom saving throw or become cursed for the duration. Until the curse ends, the target suffers one of the following effects of your choice:

•  Choose one ability. The target has Disadvantage on ability checks and saving throws made with that ability.

•  The target has Disadvantage on attack rolls against you.

•  In combat, the target must succeed on a Wisdom saving throw at the start of each of its turns or be forced to take the Dodge action on that turn.

<!-- SRD page 113 -->

•  If you deal damage to the target with an attack roll or a spell, the target takes an extra 1d8 Necrotic damage.

Using a Higher-Level Spell Slot. If you cast this spell using a level 4 spell slot, you can maintain Concentration on it for up to 10 minutes. If you use a level 5+ spell slot, the spell doesn’t require Concentration, and the duration becomes 8 hours (level 5–6 slot) or 24 hours (level 7–8 slot). If you use a level 9 spell slot, the spell lasts until dispelled.

Original layout: [[../../srd-5.2.1/pages/112|page 112]], [[../../srd-5.2.1/pages/113|page 113]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/bestow_curse ` |
| ` implementation ` | ` dnd2024:spell_bestow_curse ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` determined_by ` | ` slot_duration ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` touch ` | ` true ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` unaffected ` |

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` apply ` | ` curse ` |
| ` choose_effect ` | ` 1 ` |
| ` chooser ` | ` caster ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
ability_impairment:
  ability_choice:
  - strength
  - dexterity
  - constitution
  - intelligence
  - wisdom
  - charisma
  disadvantage:
  - chosen_ability_checks
  - chosen_ability_saves
attack_impairment:
  disadvantage: attack_rolls
  against: caster
forced_dodge:
  in_combat_only: true
  event: turn.start
  subject: target
  save:
    ability: wisdom
    dc: spell_save_dc
  failure:
    mandatory_action: dodge
    duration: current_turn
extra_damage:
  event: damage.applied
  source: caster
  target: cursed_target
  eligible_causes:
  - attack_roll
  - spell
  extra_damage:
    dice: 1d8
    type: necrotic
```

#### Slot duration

<!-- splat:table
path: splat_rule/parameters/slot_duration
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
  Levels:
    path: levels
    type: yaml
  Seconds:
    path: seconds
    type: integer
    optional: true
  Concentration:
    path: concentration
    type: boolean
  Until:
    path: until
    type: symbol
    optional: true
-->

| Key | Levels | Seconds | Concentration | Until |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` [3] ` | ` 60 ` | true |  |
| ` row_2 ` | ` [4] ` | ` 600 ` | true |  |
| ` row_3 ` | ` [5, 6] ` | ` 28800 ` | false |  |
| ` row_4 ` | ` [7, 8] ` | ` 86400 ` | false |  |
| ` row_5 ` | ` [9] ` |  | false | ` Dispelled ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

