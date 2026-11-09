---
splat_schema: 2
splat_id: b2368195-f6cb-56fe-8385-2ee5e1d60623
splat_kind: spell
splat_key: dnd2024:spells/polymorph
splat_name: Polymorph
splat_origin: dnd2024:reference/spell/polymorph
splat_sources:
- SRD 5.2.1, pp. 153, 154, https://www.dndbeyond.com/srd
---

# Polymorph


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Transmutation |
| Classes | Bard, Druid, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S, M (a caterpillar cocoon) |
| Duration | Concentration, up to 1 hour |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 153 -->

Level 4 Transmutation (Bard, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a caterpillar cocoon)\
Duration: Concentration, up to 1 hour

You attempt to transform a creature that you can see within range into a Beast. The target must succeed on a Wisdom saving throw or shape-shift into a Beast form for the duration. That form can be any Beast you choose that has a Challenge Rating equal to or less than the target’s (or the target’s level if it doesn’t have a Challenge Rating). The target’s game statistics are replaced by the stat block of the chosen Beast, but the target retains its alignment, personality, creature type, Hit Points, and Hit Point Dice. See the “Animals” section of “Monsters” for a sample of Beast stat blocks.

The target gains a number of Temporary Hit Points equal to the Hit Points of the Beast form. These Temporary Hit Points vanish if any remain when the spell ends. The spell ends early on the target if it has no Temporary Hit Points left.

The target is limited in the actions it can perform by the anatomy of its new form, and it can’t speak or cast spells.

<!-- SRD page 154 -->

The target’s gear melds into the new form. The creature can’t use or otherwise benefit from any of that equipment.

Original layout: [[../../srd-5.2.1/pages/153|page 153]], [[../../srd-5.2.1/pages/154|page 154]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/polymorph ` |
| ` implementation ` | ` dnd2024:spell_polymorph ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` failure ` | ` transform ` |
| ` success ` | ` no_effect ` |

#### Form

<!-- splat:values path=splat_rule/parameters/form -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` type ` | ` beast ` |
| ` maximum_cr ` | ` target_cr_or_level_if_no_cr ` |
| ` replace_stats ` | ` chosen_beast_stat_block ` |
| ` retain ` | ` ["alignment", "personality", "creature_type", "hit_points", "hit_point_dice"] ` |
| ` duration ` | ` spell_duration ` |

#### Temporary hp

<!-- splat:values path=splat_rule/parameters/temporary_hp -->

| Field | Value |
| --- | --- |
| ` amount ` | ` beast_form_hit_points ` |
| ` remainder_vanishes_on_spell_end ` | ` true ` |
| ` target_spell_ends_when ` | ` no_temporary_hp_remaining ` |

#### Restrictions

<!-- splat:values path=splat_rule/parameters/restrictions -->

| Field | Value |
| --- | --- |
| ` actions ` | ` new_anatomy_limits ` |
| ` speech ` | ` forbidden ` |
| ` spellcasting ` | ` forbidden ` |

#### Equipment

<!-- splat:values path=splat_rule/parameters/equipment -->

| Field | Value |
| --- | --- |
| ` melds_into_form ` | ` true ` |
| ` use ` | ` forbidden ` |
| ` benefits ` | ` none ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: hour
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

