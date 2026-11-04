---
splat_schema: 2
splat_id: 19812bef-7e04-560e-ba1d-256201a0789d
splat_kind: spell
splat_key: dnd2024:spells/contagion
splat_name: Contagion
splat_origin: dnd2024:reference/spell/contagion
splat_sources:
- SRD 5.2.1, pp. 119, https://www.dndbeyond.com/srd
---

# Contagion


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Necromancy |
| Classes | Cleric, Druid |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | 7 days |


## Source text

<!-- SRD page 119 -->

Level 5 Necromancy (Cleric, Druid)\
Casting Time: Action\
Range: Touch\
Component: V, S\
Duration: 7 days

Your touch inflicts a magical contagion. The target must succeed on a Constitution saving throw or take 11d8 Necrotic damage and have the Poisoned condition. Also, choose one ability when you cast the spell. While Poisoned, the target has Disadvantage on saving throws made with the chosen ability.

The target must repeat the saving throw at the end of each of its turns until it gets three successes or failures. If the target succeeds on three of these saves, the spell ends on the target. If the target fails three of the saves, the spell lasts for 7 days on it.

Whenever the Poisoned target receives an effect that would end the Poisoned condition, the target must succeed on a Constitution saving throw, or the Poisoned condition doesn’t end on it.

Original layout: [[../../srd-5.2.1/pages/119|page 119]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/contagion ` |
| ` implementation ` | ` dnd2024:spell_contagion ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ability_choice ` | ` ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"] ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  binding: save_dc
failure:
  damage:
    dice:
      count: 11
      sides: 8
      add: 0
    type: necrotic
  condition: poisoned
  while_poisoned:
    saving_throw_disadvantage_ability: chosen
  repeat_save:
    event: target_turn_end
    successes_to_end: 3
    failures_to_stop_repeating_and_persist: 3
    count_initial_save: false
    duration_days: 7
  poison_removal_attempt:
    save:
      ability: constitution
      dc:
        binding: save_dc
      failure:
        prevent_condition_removal: true
      success:
        allow_condition_removal: true
success:
  effect: none
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

