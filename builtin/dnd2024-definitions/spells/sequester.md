---
splat_schema: 1
splat_id: 579b889f-b6c7-53dd-b742-da6eec87361a
splat_kind: spell
splat_key: dnd2024:spells/sequester
splat_name: Sequester
splat_origin:
  key: dnd2024:reference/spell/sequester
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161
  url: https://www.dndbeyond.com/srd
---

# Sequester

## Source text

<!-- SRD page 161 -->

Level 7 Transmutation (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (gem dust worth 5,000+ GP,\
which the spell consumes)\
Duration: Until dispelled\
With a touch, you magically sequester an object or a\
willing creature. For the duration, the target has the\
Invisible condition and can’t be targeted by Divina-\
tion spells, detected by magic, or viewed remotely\
with magic.\
If the target is a creature, it enters a state of sus-\
pended animation; it has the Unconscious condition,\
doesn’t age, and doesn’t need food, water, or air.\
You can set a condition for the spell to end early.\
The condition can be anything you choose, but it\
must occur or be visible within 1 mile of the target.\
Examples include “after 1,000 years” or “when the\
tarrasque awakens.” This spell also ends if the tar-\
get takes any damage.\

Original layout: [[../../srd-5.2.1/pages/161|page 161]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Until dispelled ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` gem dust worth 5,000+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/sequester ` |
| ` implementation ` | ` dnd2024:spell_sequester ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` until ` | ` dispelled_or_source_specific_end ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` choices ` | ` ["object", "willing_creature"] ` |
| ` touch ` | ` true ` |

#### Protection

<!-- splat:values path=splat_rule/parameters/protection -->

| Field | Value |
| --- | --- |
| ` condition ` | ` invisible ` |
| ` divination_spell_targeting ` | ` forbidden ` |
| ` magical_detection ` | ` forbidden ` |
| ` magical_remote_viewing ` | ` forbidden ` |

#### Creature extra

<!-- splat:values path=splat_rule/parameters/creature_extra -->

| Field | Value |
| --- | --- |
| ` state ` | ` suspended_animation ` |
| ` condition ` | ` unconscious ` |
| ` aging ` | ` false ` |
| ` food_required ` | ` false ` |
| ` water_required ` | ` false ` |
| ` air_required ` | ` false ` |

#### Optional end trigger

<!-- splat:values path=splat_rule/parameters/optional_end_trigger -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` requirement ` | ` occurs_or_visible_within_target_miles ` |
| ` maximum_miles ` | ` 1 ` |
| ` supports_elapsed_time ` | ` true ` |
| ` condition_resolution ` | ` supplied_trigger_predicate ` |

#### End on damage

<!-- splat:values path=splat_rule/parameters/end_on_damage -->

| Field | Value |
| --- | --- |
| ` event ` | ` damage.applied ` |
| ` subject ` | ` target ` |

#### Target binding

<!-- splat:values path=splat_rule/parameters/target_binding -->

| Field | Value |
| --- | --- |
| ` exactly_one ` | ` ["target_creature", "target_object"] ` |
| ` must_match_selected_mode ` | ` true ` |
