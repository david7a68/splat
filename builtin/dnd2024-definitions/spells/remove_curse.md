---
splat_schema: 1
splat_id: 4d28de7e-cb68-587c-8897-1ccc428b74a1
splat_kind: spell
splat_key: dnd2024:spells/remove_curse
splat_name: Remove Curse
splat_origin:
  key: dnd2024:reference/spell/remove_curse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
---

# Remove Curse

## Source text

<!-- SRD page 158 -->

Level 3 Abjuration (Cleric, Paladin, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous

At your touch, all curses affecting one creature or object end. If the object is a cursed magic item, its curse remains, but the spell breaks its owner’s Attunement to the object so it can be removed or discarded.

Original layout: [[../../srd-5.2.1/pages/158|page 158]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "paladin", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/remove_curse ` |
| ` implementation ` | ` dnd2024:spell_remove_curse ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kinds ` | ` ["creature", "object"] ` |
| ` touch ` | ` true ` |
| ` count ` | ` 1 ` |

#### Normal

<!-- splat:values path=splat_rule/parameters/normal -->

| Field | Value |
| --- | --- |
| ` end ` | ` all_curses_affecting_target ` |

#### Cursed magic item exception

<!-- splat:values path=splat_rule/parameters/cursed_magic_item_exception -->

| Field | Value |
| --- | --- |
| ` item_curse ` | ` remains ` |
| ` owner_attunement ` | ` broken ` |
| ` item_removal_and_discard ` | ` permitted ` |

#### Target binding

<!-- splat:values path=splat_rule/parameters/target_binding -->

| Field | Value |
| --- | --- |
| ` exactly_one ` | ` ["target_creature", "target_object"] ` |
| ` must_match_selected_mode ` | ` true ` |
