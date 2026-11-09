---
splat_schema: 2
splat_id: ed4eb4d9-23b3-5629-942a-66ce2d688863
splat_kind: spell
splat_key: dnd2024:spells/planar_binding
splat_name: Planar Binding
splat_origin: dnd2024:reference/spell/planar_binding
splat_sources:
- SRD 5.2.1, pp. 152, 153, https://www.dndbeyond.com/srd
---

# Planar Binding


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Abjuration |
| Classes | Bard, Cleric, Druid, Warlock, Wizard |
| Casting Time | 1 hour |
| Range | 60 feet |
| Components | V, S, M (a jewel worth 1,000+ GP, which the spell consumes) |
| Duration | 24 hours |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 152 -->

Level 5 Abjuration (Bard, Cleric, Druid, Warlock, Wizard)\
Casting Time: 1 hour\
Range: 60 feet\
Components: V, S, M (a jewel worth 1,000+ GP, which the spell consumes)\
Duration: 24 hours

<!-- SRD page 153 -->

You attempt to bind a Celestial, an Elemental, a Fey, or a Fiend to your service. The creature must be within range for the entire casting of the spell. (Typically, the creature is first summoned into the center of the inverted version of the Magic Circle spell to trap it while this spell is cast.) At the completion of the casting, the target must succeed on a Charisma saving throw or be bound to serve you for the duration. If the creature was summoned or created by another spell, that spell’s duration is extended to match the duration of this spell.

A bound creature must follow your commands to the best of its ability. You might command the creature to accompany you on an adventure, to guard a location, or to deliver a message. If the creature is Hostile, it strives to twist your commands to achieve its own objectives. If the creature carries out your commands completely before the spell ends, it travels to you to report this fact if you are on the same plane of existence. If you are on a different plane, it returns to the place where you bound it and remains there until the spell ends.

Using a Higher-Level Spell Slot. The duration increases with a spell slot of level 6 (10 days), 7 (30 days), 8 (180 days), and 9 (366 days).

Original layout: [[../../srd-5.2.1/pages/152|page 152]], [[../../srd-5.2.1/pages/153|page 153]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/planar_binding ` |
| ` implementation ` | ` dnd2024:spell_planar_binding ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` types ` | ` ["celestial", "elemental", "fey", "fiend"] ` |
| ` within_range_for_entire_cast ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
timing: casting_completion
ability: charisma
failure:
  bound_service: spell_duration
  summoned_or_created_by_other_spell: extend_that_spell_to_this_duration
```

#### Commands

<!-- splat:values path=splat_rule/parameters/commands -->

| Field | Value |
| --- | --- |
| ` obedience ` | ` best_of_ability ` |
| ` hostile_target ` | ` twists_commands_toward_own_objectives ` |

#### Completed commands early

<!-- splat:values path=splat_rule/parameters/completed_commands_early -->

| Field | Value |
| --- | --- |
| ` same_plane ` | ` travel_to_caster_and_report ` |
| ` different_plane ` | ` return_to_binding_location_and_wait_until_spell_end ` |

#### Slot duration days

<!-- splat:yaml path=splat_rule/parameters/slot_duration_days -->

```yaml
'5': 1
'6': 10
'7': 30
'8': 180
'9': 366
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 24
  unit: hour
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> ```

