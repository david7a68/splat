---
splat_schema: 2
splat_id: 54e4346b-9165-5413-a574-d02241065131
splat_kind: spell
splat_key: dnd2024:spells/conjure_animals
splat_name: Conjure Animals
splat_origin: dnd2024:reference/spell/conjure_animals
splat_sources:
- SRD 5.2.1, pp. 117, 118, https://www.dndbeyond.com/srd
---

# Conjure Animals


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Conjuration |
| Classes | Druid, Ranger |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 117 -->

Level 3 Conjuration (Druid, Ranger)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes

You conjure nature spirits that appear as a Large pack of spectral, intangible animals in an unoccupied space you can see within range. The pack lasts for the duration, and you choose the spirits’ animal form, such as wolves, serpents, or birds.

You have Advantage on Strength saving throws while you’re within 5 feet of the pack, and when you move on your turn, you can also move the pack up to 30 feet to an unoccupied space you can see.

Whenever the pack moves within 10 feet of a creature you can see and whenever a creature you can see enters a space within 10 feet of the pack or ends its turn there, you can force that creature to make a Dexterity saving throw. On a failed save, the creature takes 3d10 Slashing damage. A creature makes this save only once per turn.

<!-- SRD page 118 -->

Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot level above 3.

Original layout: [[../../srd-5.2.1/pages/117|page 117]], [[../../srd-5.2.1/pages/118|page 118]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/conjure_animals ` |
| ` implementation ` | ` dnd2024:spell_conjure_animals ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 600 ` |

#### Pack

<!-- splat:yaml path=splat_rule/parameters/pack -->

```yaml
size: large
spectral: true
tangible: false
initial_space:
  unoccupied: true
  visible: true
  range_feet: 60
animal_appearance:
  chooser: caster
```

#### Caster save benefit

<!-- splat:values path=splat_rule/parameters/caster_save_benefit -->

| Field | Value |
| --- | --- |
| ` ability ` | ` strength ` |
| ` advantage ` | ` true ` |
| ` within_pack_feet ` | ` 5 ` |

#### Movement

<!-- splat:yaml path=splat_rule/parameters/movement -->

```yaml
event: participant.moved
subject: caster
caster_turn_only: true
optional: true
maximum_feet: 30
destination:
  unoccupied: true
  visible_to_caster: true
```

#### Damage trigger

<!-- splat:yaml path=splat_rule/parameters/damage_trigger -->

```yaml
events:
- pack.moved_within_distance
- creature.entered_distance
- turn.end
distance_feet: 10
target_kind: creature
target_visible_to_caster: true
chooser: caster
optional: true
save_limit:
  count: 1
  per: turn
  scope: target
```

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` no_damage ` |

#### Failure damage

<!-- splat:yaml path=splat_rule/parameters/failure_damage -->

```yaml
dice:
  base_count: 3
  sides: 10
  extra_per_slot: 1
  base_slot: 3
type: slashing
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
