---
splat_schema: 1
splat_id: c2ec689e-084d-513b-80ff-c14e8b91a1dc
splat_kind: spell
splat_key: dnd2024:spells/create_food_and_water
splat_name: Create Food and Water
splat_origin:
  key: dnd2024:reference/spell/create_food_and_water
splat_sources:
- title: SRD 5.2.1
  locator: pp. 120
  url: https://www.dndbeyond.com/srd
---

# Create Food and Water

## Source text

<!-- SRD page 120 -->

Level 3 Conjuration (Cleric, Paladin)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Instantaneous\
You create 45 pounds of food and 30 gallons of\
fresh water on the ground or in containers within\
range—both useful in fending off the hazards of\
malnutrition and dehydration. The food is bland but\
nourishing and looks like a food of your choice, and\
the water is clean. The food spoils after 24 hours if\
uneaten.\

Original layout: [[../../srd-5.2.1/pages/120|page 120]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["cleric", "paladin"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/create_food_and_water ` |
| ` implementation ` | ` dnd2024:spell_create_food_and_water ` |

### Parameters

#### Placement

<!-- splat:values path=splat_rule/parameters/placement -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["ground", "containers"] ` |
| ` within_range ` | ` true ` |

#### Food

<!-- splat:values path=splat_rule/parameters/food -->

| Field | Value |
| --- | --- |
| ` pounds ` | ` 45 ` |
| ` nourishing ` | ` true ` |
| ` taste ` | ` bland ` |
| ` appearance_choice ` | ` food_kind ` |
| ` uneaten_spoils_hours ` | ` 24 ` |

#### Water

<!-- splat:values path=splat_rule/parameters/water -->

| Field | Value |
| --- | --- |
| ` gallons ` | ` 30 ` |
| ` fresh ` | ` true ` |
| ` clean ` | ` true ` |
