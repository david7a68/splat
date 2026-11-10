---
splat_schema: 2
splat_id: 4798c2d4-9515-5463-91a6-1adeff908ce8
splat_kind: spell
splat_key: dnd2024:spells/programmed_illusion
splat_name: Programmed Illusion
splat_origin: dnd2024:reference/spell/programmed_illusion
splat_sources:
- SRD 5.2.1, pp. 156, https://www.dndbeyond.com/srd
---

# Programmed Illusion


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Illusion |
| Classes | Bard, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S, M (jade dust worth 25+ GP) |
| Duration | Until dispelled |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 156 -->

Level 6 Illusion (Bard, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (jade dust worth 25+ GP)\
Duration: Until dispelled

You create an illusion of an object, a creature, or some other visible phenomenon within range that activates when a specific trigger occurs. The illusion is imperceptible until then. It must be no larger than a 30-foot Cube, and you decide when you cast the spell how the illusion behaves and what sounds it makes. This scripted performance can last up to 5 minutes.

When the trigger you specify occurs, the illusion springs into existence and performs in the manner you described. Once the illusion finishes performing, it disappears and remains dormant for 10 minutes, after which the illusion can be activated again.

The trigger can be as general or as detailed as you like, though it must be based on visual or audible phenomena that occur within 30 feet of the area. For example, you could create an illusion of yourself to appear and warn off others who attempt to open a trapped door.

Physical interaction with the image reveals it to be illusory, since things can pass through it. A creature that takes the Study action to examine the image can determine that it is an illusion with a successful Intelligence (Investigation) check against your spell save DC. If a creature discerns the illusion for what it is, the creature can see through the image, and any noise it makes sounds hollow to the creature.

Original layout: [[../../srd-5.2.1/pages/156|page 156]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/programmed_illusion ` |
| ` implementation ` | ` dnd2024:spell_programmed_illusion ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` until ` | ` dispelled_or_source_specific_end ` |

#### Illusion

<!-- splat:values path=splat_rule/parameters/illusion -->

| Field | Value |
| --- | --- |
| ` kinds ` | ` ["object", "creature", "visible_phenomenon"] ` |
| ` range_feet ` | ` 120 ` |
| ` maximum_cube_side_feet ` | ` 30 ` |
| ` initial_state ` | ` imperceptible ` |
| ` visual_behavior ` | ` caster_script_at_cast ` |
| ` sounds ` | ` caster_script_at_cast ` |
| ` performance_seconds_max ` | ` 300 ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` timing ` | ` cast ` |
| ` modalities ` | ` ["visual", "audible"] ` |
| ` maximum_distance_from_area_feet ` | ` 30 ` |
| ` condition_resolution ` | ` supplied_trigger_predicate ` |

#### Cycle

<!-- splat:yaml path=splat_rule/parameters/cycle -->

```yaml
on_trigger: perform_script
after_performance:
  disappear: true
  dormant_seconds: 600
after_dormancy: triggerable_again
```

#### Physical interaction

<!-- splat:values path=splat_rule/parameters/physical_interaction -->

| Field | Value |
| --- | --- |
| ` objects_pass_through ` | ` true ` |
| ` reveals_illusion ` | ` true ` |

#### Examination

<!-- splat:yaml path=splat_rule/parameters/examination -->

```yaml
economy: study_action
check:
  ability: intelligence
  skill: investigation
  dc: spell_save_dc
success: discern_illusion
```

#### Discerned

<!-- splat:values path=splat_rule/parameters/discerned -->

| Field | Value |
| --- | --- |
| ` image ` | ` see_through ` |
| ` noise ` | ` hollow ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

