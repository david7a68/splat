---
splat_schema: 1
splat_id: 389d6bd9-cf4b-5557-82f9-27aa039b1fa7
splat_kind: item
splat_key: dnd2024:items/sphere_of_annihilation
splat_name: Sphere of Annihilation
splat_origin:
  key: dnd2024:reference/magic_item/sphere_of_annihilation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 244, 245
  url: https://www.dndbeyond.com/srd
---

# Sphere of Annihilation

## Source text

<!-- SRD page 244 -->

Wondrous Item, Legendary

This 2-foot-diameter black sphere is a hole in the multiverse, hovering in space and stabilized by a magical field surrounding it.

The sphere obliterates all matter it passes through and all matter that passes through it. Artifacts are the exception. Unless an Artifact is susceptible to damage from a Sphere of Annihilation, it passes through the sphere unscathed. Anything else that touches the sphere but isn’t wholly engulfed and obliterated by it takes 8d10 Force damage.

Controlling the Sphere. A Sphere of Annihilation is stationary until someone takes control of it. If you are within 60 feet of a sphere, you can take a Magic action to make a DC 25 Intelligence (Arcana) check. On a successful check, you control the sphere until the start of your next turn, and if it was under another creature’s control, that creature loses control of the sphere. On a failed check, the sphere moves 10 feet toward you in a straight line.

While in control of the sphere, you can take a Bonus Action to cause it to move in one direction of your choice, up to a number of feet equal to 5 times your Intelligence modifier (minimum 5 feet). Any creature whose space the sphere enters must succeed on a DC 19 Dexterity saving throw or be touched by it, taking 8d10 Force damage. A creature reduced to 0 Hit Points by this damage is obliterated, leaving its possessions behind but no other physical remains.

Sphere Interactions. If the sphere comes into contact with a planar portal (such as that created by the Gate spell) or an extradimensional space (such as that within a Portable Hole), the GM determines randomly what happens using the following table.

<!-- SRD page 245 -->

| 1d100 | Result |
| --- | --- |
| 01–50 | The sphere is destroyed. |
| 51–85 | The sphere moves through the portal or into the extradimensional space. |
| 86–00 | A spatial rift sends the sphere and each creature and object within 180 feet of the sphere to a random plane of existence. |

Original layout: [[../../srd-5.2.1/pages/244|page 244]], [[../../srd-5.2.1/pages/245|page 245]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/sphere_of_annihilation ` |
| ` implementation ` | ` dnd2024:item_sphere_of_annihilation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` diameter_ft ` | ` 2 ` |
| ` hovering ` | ` true ` |
| ` default_motion ` | ` stationary ` |
| ` attunement_required ` | ` false ` |

#### Contact

<!-- splat:yaml path=splat_rule/parameters/contact -->

```yaml
wholly_engulfed: matter_obliterated
partly_touching:
  damage:
    amount:
      roll:
        count: 8
        sides: 10
    type: force
artifact_exception: unharmed_unless_artifact_explicitly_susceptible
```

#### Control

<!-- splat:yaml path=splat_rule/parameters/control -->

```yaml
action: magic
maximum_range_ft: 60
check:
  ability: intelligence
  skill: arcana
  dc: 25
  success:
    control_until: user.next_turn_start
    previous_controller_loses_control: true
  failure:
    move_ft: 10
    direction: straight_toward_user
```

#### Controlled movement

<!-- splat:yaml path=splat_rule/parameters/controlled_movement -->

```yaml
action: bonus_action
direction_choice: one_direction
maximum_distance_ft:
  max:
  - 5
  - multiply:
    - 5
    - user.intelligence_modifier
```

#### Interaction

<!-- splat:values path=splat_rule/parameters/interaction -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` ["planar_portal_contact", "extradimensional_space_contact"] ` |
| ` die ` | ` 100 ` |

##### Rows

<!-- splat:table
path: splat_rule/parameters/interaction/rows
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Range:
    path: range
    type: yaml
  Destroy sphere:
    path: destroy_sphere
    type: boolean
    optional: true
  Sphere:
    path: sphere
    type: symbol
    optional: true
  Transport:
    path: transport
    type: yaml
    optional: true
  Radius ft:
    path: radius_ft
    type: integer
    optional: true
  Destination plane:
    path: destination_plane
    type: symbol
    optional: true
-->

| Key | Range | Destroy sphere | Sphere | Transport | Radius ft | Destination plane |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` [1, 50] ` | true |  |  |  |  |
| ` row_2 ` | ` [51, 85] ` |  | ` Passes through or enters ` |  |  |  |
| ` row_3 ` | ` [86, 100] ` |  |  | ` ["sphere", "creatures", "objects"] ` | ` 180 ` | ` Random ` |

#### On any sphere movement into creature space

<!-- splat:yaml path=splat_rule/parameters/on_any_sphere_movement_into_creature_space -->

```yaml
save:
  ability: dexterity
  dc: 19
  failure:
    damage:
      amount:
        roll:
          count: 8
          sides: 10
      type: force
zero_hp:
  obliterate_body: true
  leave_possessions: true
  physical_remains: false
```
