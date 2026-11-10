---
splat_schema: 2
splat_id: fdda293d-3ffe-5a92-8eb6-9e7c15474c40
splat_kind: spell
splat_key: dnd2024:spells/enhance_ability
splat_name: Enhance Ability
splat_origin: dnd2024:reference/spell/enhance_ability
splat_sources:
- SRD 5.2.1, pp. 127, https://www.dndbeyond.com/srd
---

# Enhance Ability


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Transmutation |
| Classes | Bard, Cleric, Druid, Ranger, Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (fur or a feather) |
| Duration | Concentration, up to 1 hour |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 127 -->

Level 2 Transmutation (Bard, Cleric, Druid, Ranger, Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (fur or a feather)\
Duration: Concentration, up to 1 hour

You touch a creature and choose Strength, Dexterity, Intelligence, Wisdom, or Charisma. For the duration, the target has Advantage on ability checks using the chosen ability.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 2. You can choose a different ability for each target.

Original layout: [[../../srd-5.2.1/pages/127|page 127]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/enhance_ability ` |
| ` implementation ` | ` dnd2024:spell_enhance_ability ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
touch: true
count:
  base: 1
  increment_per_slot_above: 1
  base_slot: 2
```

#### Ability

<!-- splat:values path=splat_rule/parameters/ability -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` per_target ` | ` true ` |
| ` options ` | ` ["strength", "dexterity", "intelligence", "wisdom", "charisma"] ` |

#### Modifier

<!-- splat:values path=splat_rule/parameters/modifier -->

| Field | Value |
| --- | --- |
| ` selector ` | ` ability_checks_using_chosen_ability ` |
| ` advantage ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

