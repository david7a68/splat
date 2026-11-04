---
splat_schema: 2
splat_id: 3214d5d0-ab3f-5d2f-a4dc-27dc268a48b2
splat_kind: spell
splat_key: dnd2024:spells/sanctuary
splat_name: Sanctuary
splat_origin: dnd2024:reference/spell/sanctuary
splat_sources:
- SRD 5.2.1, pp. 159, https://www.dndbeyond.com/srd
---

# Sanctuary


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Abjuration |
| Classes | Cleric |
| Casting Time | Bonus Action |
| Range | 30 feet |
| Components | V, S, M (a shard of glass from a mirror) |
| Duration | 1 minute |


## Source text

<!-- SRD page 159 -->

Level 1 Abjuration (Cleric)\
Casting Time: Bonus Action\
Range: 30 feet\
Components: V, S, M (a shard of glass from a mirror)\
Duration: 1 minute

You ward a creature within range. Until the spell ends, any creature who targets the warded creature with an attack roll or a damaging spell must succeed on a Wisdom saving throw or either choose a new target or lose the attack or spell. This spell doesn’t protect the warded creature from areas of effect. The spell ends if the warded creature makes an attack roll, casts a spell, or deals damage.

Original layout: [[../../srd-5.2.1/pages/159|page 159]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/sanctuary ` |
| ` implementation ` | ` dnd2024:spell_sanctuary ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` end_early_events ` | ` ["warded_creature.made_attack_roll", "warded_creature.cast_spell", "warded_creature.dealt_damage"] ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 60 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` range_feet ` | ` 30 ` |

#### Protective trigger

<!-- splat:values path=splat_rule/parameters/protective_trigger -->

| Field | Value |
| --- | --- |
| ` event ` | ` target.selected ` |
| ` warded_creature_targeted_by ` | ` ["attack_roll", "damaging_spell"] ` |
| ` attacker_kind ` | ` creature ` |
| ` exclude ` | ` area_of_effect ` |

#### Attacker save

<!-- splat:yaml path=splat_rule/parameters/attacker_save -->

```yaml
ability: wisdom
dc: spell_save_dc
success: original_target_allowed
failure:
  chooser: attacker
  options:
  - choose_new_legal_target
  - lose_attack_or_spell
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

