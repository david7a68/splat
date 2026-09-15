---
splat_schema: 1
splat_id: ef52665e-5eb6-575b-a612-dcd08c65f661
splat_kind: rule
splat_key: dnd2024:rules/character_species
splat_name: Character Species
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
---

# Character Species

## Source text

<!-- SRD page 83 -->

When you choose your character’s species, you\
determine whether your character is a human or a\
member of a fantastical species, such as dragonborn\
or gnome.\
The peoples of the multiverse hail from different\
worlds and include many kinds of sapient life forms.\
A player character’s species is the set of game traits\
that an adventurer gains from being one of those\
life forms.\
Some species can trace their origin to a single\
world, plane of existence, or god, while other spe-\
cies first appeared in multiple realms at once.\
Whatever a species’ genesis, its members have\
spread across the multiverse and contribute to\
many different cultures.\
Members of most species live for about 80 years,\
with exceptions noted in the text about the spe-\
cies in “Character Origins.” Regardless of life span,\
members of all species reach physical maturity at\
about the same age. Your character can be any age\
that isn’t beyond their species’ normal life span.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/character_species ` |
| ` implementation ` | ` dnd2024:rules_character_species ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` normal_lifespan_years_default ` | ` 80 ` |
| ` species_lifespan_overrides_default ` | ` true ` |
| ` physical_maturity_age_basis ` | ` shared_across_species ` |

#### Species selection

<!-- splat:values path=splat_rule/parameters/species_selection -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` grants ` | ` species_game_traits ` |

#### Character age

<!-- splat:values path=splat_rule/parameters/character_age -->

| Field | Value |
| --- | --- |
| ` max ` | ` species_normal_lifespan ` |
