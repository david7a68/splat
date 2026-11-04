---
splat_schema: 2
splat_id: 6965e670-1196-540c-89e2-18dc4b69b893
splat_kind: spell
splat_key: dnd2024:spells/teleportation_circle
splat_name: Teleportation Circle
splat_origin: dnd2024:reference/spell/teleportation_circle
splat_sources:
- SRD 5.2.1, pp. 168, 169, https://www.dndbeyond.com/srd
---

# Teleportation Circle


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Conjuration |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | 1 minute |
| Range | 10 feet |
| Components | V, M (rare inks worth 50+ GP, which the spell consumes) |
| Duration | 1 round |


## Source text

<!-- SRD page 168 -->

Level 5 Conjuration (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: 1 minute\
Range: 10 feet\
Components: V, M (rare inks worth 50+ GP, which the spell consumes)\
Duration: 1 round

As you cast the spell, you draw a 5-foot-radius circle on the ground inscribed with sigils that link your location to a permanent teleportation circle of your choice whose sigil sequence you know and that is on the same plane of existence as you. A shimmering portal opens within the circle you drew and remains open until the end of your next turn. Any creature that enters the portal instantly appears within 5 feet of the destination circle or in the nearest unoccupied space if that space is occupied.

<!-- SRD page 169 -->

Many major temples, guildhalls, and other important places have permanent teleportation circles. Each circle includes a unique sigil sequence—a string of runes arranged in a particular pattern.

When you first gain the ability to cast this spell, you learn the sigil sequences for two destinations on the Material Plane, determined by the GM. You might learn additional sigil sequences during your adventures. You can commit a new sigil sequence to memory after studying it for 1 minute.

You can create a permanent teleportation circle by casting this spell in the same location every day for 365 days.

Original layout: [[../../srd-5.2.1/pages/168|page 168]], [[../../srd-5.2.1/pages/169|page 169]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/teleportation_circle ` |
| ` implementation ` | ` dnd2024:spell_teleportation_circle ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_rounds ` | ` 1 ` |

#### Origin

<!-- splat:values path=splat_rule/parameters/origin -->

| Field | Value |
| --- | --- |
| ` ground ` | ` true ` |
| ` range_feet ` | ` 10 ` |
| ` circle_radius_feet ` | ` 5 ` |
| ` draw_sigils_during_cast ` | ` true ` |

#### Destination

<!-- splat:values path=splat_rule/parameters/destination -->

| Field | Value |
| --- | --- |
| ` permanent_teleportation_circle ` | ` true ` |
| ` sigil_sequence_known ` | ` true ` |
| ` same_plane ` | ` true ` |
| ` chooser ` | ` caster ` |

#### Portal

<!-- splat:yaml path=splat_rule/parameters/portal -->

```yaml
until:
  event: turn.end
  subject: caster
  occurrence: next
trigger: creature.entered_portal
transport: instantaneous
arrival_within_destination_feet: 5
occupied_fallback: nearest_unoccupied_space
```

#### Knowledge

<!-- splat:yaml path=splat_rule/parameters/knowledge -->

```yaml
on_first_gain_casting_ability:
  learn_sequences: 2
  destination_plane: material
  chooser: gm
additional_sequence_study_seconds: 60
sigil_sequences: unique_per_circle
```

#### Permanence

<!-- splat:values path=splat_rule/parameters/permanence -->

| Field | Value |
| --- | --- |
| ` same_location ` | ` true ` |
| ` frequency ` | ` daily ` |
| ` cast_days ` | ` 365 ` |
| ` result ` | ` permanent_teleportation_circle ` |

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
>   consumed: true
> ```

