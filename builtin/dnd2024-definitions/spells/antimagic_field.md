---
splat_schema: 2
splat_id: 82c2be54-7083-5c43-9d69-917d6f78556e
splat_kind: spell
splat_key: dnd2024:spells/antimagic_field
splat_name: Antimagic Field
splat_origin: dnd2024:reference/spell/antimagic_field
splat_sources:
- SRD 5.2.1, pp. 109, https://www.dndbeyond.com/srd
---

# Antimagic Field


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Abjuration |
| Classes | Cleric, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (iron filings) |
| Duration | Concentration, up to 1 hour |


## Source text

<!-- SRD page 109 -->

Level 8 Abjuration (Cleric, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (iron filings)\
Duration: Concentration, up to 1 hour

An aura of antimagic surrounds you in 10-foot Emanation. No one can cast spells, take Magic actions, or create other magical effects inside the aura, and those things can’t target or otherwise affect anything inside it. Magical properties of magic items don’t work inside the aura or on anything inside it.

Areas of effect created by spells or other magic can’t extend into the aura, and no one can teleport into or out of it or use planar travel there. Portals close temporarily while in the aura.

Ongoing spells, except those cast by an Artifact or a deity, are suppressed in the area. While an effect is suppressed, it doesn’t function, but the time it spends suppressed counts against its duration.

Dispel Magic has no effect on the aura, and the auras created by different Antimagic Field spells don’t nullify each other.

Original layout: [[../../srd-5.2.1/pages/109|page 109]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/antimagic_field ` |
| ` implementation ` | ` dnd2024:spell_antimagic_field ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` forbid_inside ` | ` ["cast_spells", "magic_actions", "create_magical_effects", "teleport_in", "teleport_out", "planar_travel"] ` |
| ` prevent_affecting_inside ` | ` ["spells", "magic_actions", "magical_effects", "magic_item_properties"] ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 10 ` |
| ` origin ` | ` caster ` |
| ` moves_with_origin ` | ` true ` |

#### Suppress inside

<!-- splat:values path=splat_rule/parameters/suppress_inside -->

| Field | Value |
| --- | --- |
| ` magic_item_properties ` | ` true ` |
| ` portals ` | ` temporarily_closed ` |
| ` spell_and_magic_areas ` | ` excluded ` |

#### Ongoing spells

<!-- splat:values path=splat_rule/parameters/ongoing_spells -->

| Field | Value |
| --- | --- |
| ` operation ` | ` suppress ` |
| ` exceptions_by_caster ` | ` ["artifact", "deity"] ` |
| ` duration_elapses_while_suppressed ` | ` true ` |
| ` suppress_does_not_end ` | ` true ` |

#### Exceptions

<!-- splat:values path=splat_rule/parameters/exceptions -->

| Field | Value |
| --- | --- |
| ` dispel_magic_affects_aura ` | ` false ` |
| ` other_antimagic_fields_nullify_aura ` | ` false ` |

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

