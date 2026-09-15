---
splat_schema: 1
splat_id: dd34418b-5021-57cc-b928-0e9d53f175d4
splat_kind: spell
splat_key: dnd2024:spells/gate
splat_name: Gate
splat_origin:
  key: dnd2024:reference/spell/gate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 135
  url: https://www.dndbeyond.com/srd
---

# Gate

## Source text

<!-- SRD page 135 -->

Level 9 Conjuration (Cleric, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a diamond worth 5,000+ GP)\
Duration: Concentration, up to 1 minute\
You conjure a portal linking an unoccupied space\
you can see within range to a precise location on a\
different plane of existence. The portal is a circular\
opening, which you can make 5 to 20 feet in diam-\
eter. You can orient the portal in any direction you\
choose. The portal lasts for the duration, and the\
portal’s destination is visible through it.\
The portal has a front and a back on each plane\
where it appears. Travel through the portal is possi-\
ble only by moving through its front. Anything that\
does so is instantly transported to the other plane,\
appearing in the unoccupied space nearest to the\
portal.\
Deities and other planar rulers can prevent por-\
tals created by this spell from opening in their pres-\
ence or anywhere within their domains.\
When you cast this spell, you can speak the name\
of a specific creature (a pseudonym, title, or nick-\
name doesn’t work). If that creature is on a plane\
other than the one you are on, the portal opens\
next to the named creature and transports it to the\
nearest unoccupied space on your side of the portal.\
You gain no special power over the creature, and it\
is free to act as the GM deems appropriate. It might\
leave, attack you, or help you.\

Original layout: [[../../srd-5.2.1/pages/135|page 135]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 9 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["cleric", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a diamond worth 5,000+ GP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/gate ` |
| ` implementation ` | ` dnd2024:spell_gate ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Portal

<!-- splat:yaml path=splat_rule/parameters/portal -->

```yaml
origin:
  unoccupied: true
  visible: true
  range_feet: 60
destination:
  precise_location: true
  different_plane: true
shape: circle
diameter_feet:
  minimum: 5
  maximum: 20
  chooser: caster
orientation:
  chooser: caster
destination_visible_through_portal: true
faces:
- front
- back
```

#### Traversal

<!-- splat:values path=splat_rule/parameters/traversal -->

| Field | Value |
| --- | --- |
| ` entry_face ` | ` front_only ` |
| ` entity ` | ` anything ` |
| ` timing ` | ` instantaneous ` |
| ` destination ` | ` nearest_unoccupied_space_to_other_portal ` |

#### Planar ruler veto

<!-- splat:values path=splat_rule/parameters/planar_ruler_veto -->

| Field | Value |
| --- | --- |
| ` actors ` | ` ["deity", "planar_ruler"] ` |
| ` locations ` | ` ["in_presence", "within_domain"] ` |
| ` choice ` | ` prevent_portal_opening ` |

#### Named creature option

<!-- splat:yaml path=splat_rule/parameters/named_creature_option -->

```yaml
timing: cast
name_kind: actual_name
forbidden_name_kinds:
- pseudonym
- title
- nickname
requires: target_on_different_plane
portal_destination: next_to_named_creature
forced_transport: nearest_unoccupied_space_on_caster_side
control_granted: false
subsequent_behavior:
  chooser: gm
  actor: summoned_creature
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
