---
splat_schema: 2
splat_id: 429ab81d-cc32-5595-aeeb-6901f5462992
splat_kind: spell
splat_key: dnd2024:spells/fear
splat_name: Fear
splat_origin: dnd2024:reference/spell/fear
splat_sources:
- SRD 5.2.1, pp. 130, https://www.dndbeyond.com/srd
---

# Fear


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Illusion |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a white feather) |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 130 -->

Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a white feather)\
Duration: Concentration, up to 1 minute

Each creature in a 30-foot Cone must succeed on a Wisdom saving throw or drop whatever it is holding and have the Frightened condition for the duration.

A Frightened creature takes the Dash action and moves away from you by the safest route on each of its turns unless there is nowhere to move. If the creature ends its turn in a space where it doesn’t have line of sight to you, the creature makes a Wisdom saving throw. On a successful save, the spell ends on that creature.

Original layout: [[../../srd-5.2.1/pages/130|page 130]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/fear ` |
| ` implementation ` | ` dnd2024:spell_fear ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cone ` |
| ` length_feet ` | ` 30 ` |
| ` origin ` | ` caster ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` selection ` | ` all_in_area ` |

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
| ` drop ` | ` all_held_items ` |
| ` condition ` | ` frightened ` |
| ` source ` | ` caster ` |

#### While frightened

<!-- splat:values path=splat_rule/parameters/while_frightened -->

| Field | Value |
| --- | --- |
| ` timing ` | ` target_turn ` |
| ` action ` | ` dash ` |
| ` movement ` | ` away_from_caster ` |
| ` route ` | ` safest ` |
| ` exception ` | ` nowhere_to_move ` |
| ` route_assumption ` | ` supplied_spatial_paths ` |

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.end ` |
| ` subject ` | ` target ` |
| ` requires ` | ` no_line_of_sight_to_caster ` |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` end_on_target ` |

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
> range:
>   mode: self
> ```

