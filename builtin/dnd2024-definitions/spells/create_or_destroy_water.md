---
splat_schema: 1
splat_id: 09578d72-3c2f-562e-a002-15dd09897499
splat_kind: spell
splat_key: dnd2024:spells/create_or_destroy_water
splat_name: Create or Destroy Water
splat_origin:
  key: dnd2024:reference/spell/create_or_destroy_water
splat_sources:
- title: SRD 5.2.1
  locator: pp. 121
  url: https://www.dndbeyond.com/srd
---

# Create or Destroy Water

## Source text

<!-- SRD page 121 -->

Level 1 Transmutation (Cleric, Druid)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a mix of water and sand)\
Duration: Instantaneous\
You do one of the following:\
Create Water. You create up to 10 gallons of clean\
water within range in an open container. Alter-\
natively, the water falls as rain in a 30-foot Cube\
within range, extinguishing exposed flames there.\
Destroy Water. You destroy up to 10 gallons of\
water in an open container within range. Alter-\
natively, you destroy fog in a 30-foot Cube within\
range.\
Using a Higher-Level Spell Slot. You create or\
destroy 10 additional gallons of water, or the size of\
the Cube increases by 5 feet, for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/121|page 121]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["cleric", "druid"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a mix of water and sand ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/create_or_destroy_water ` |
| ` implementation ` | ` dnd2024:spell_create_or_destroy_water ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["create_container_water", "rain", "destroy_container_water", "destroy_fog"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` within_range ` | ` true ` |

#### Create container water

<!-- splat:values path=splat_rule/parameters/create_container_water -->

| Field | Value |
| --- | --- |
| ` container ` | ` open ` |
| ` clean ` | ` true ` |
| ` max_gallons ` | ` 10 ` |

#### Rain

<!-- splat:values path=splat_rule/parameters/rain -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cube ` |
| ` side_feet ` | ` 30 ` |
| ` extinguishes ` | ` exposed_flames ` |

#### Destroy container water

<!-- splat:values path=splat_rule/parameters/destroy_container_water -->

| Field | Value |
| --- | --- |
| ` container ` | ` open ` |
| ` max_gallons ` | ` 10 ` |

#### Destroy fog

<!-- splat:values path=splat_rule/parameters/destroy_fog -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cube ` |
| ` side_feet ` | ` 30 ` |

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 1 ` |
| ` gallons_per_level ` | ` 10 ` |
| ` cube_side_feet_per_level ` | ` 5 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
