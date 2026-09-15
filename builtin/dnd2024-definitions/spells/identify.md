---
splat_schema: 1
splat_id: 962403d3-4ca7-58f0-a9f5-debe7fdb4788
splat_kind: spell
splat_key: dnd2024:spells/identify
splat_name: Identify
splat_origin:
  key: dnd2024:reference/spell/identify
splat_sources:
- title: SRD 5.2.1
  locator: pp. 142
  url: https://www.dndbeyond.com/srd
---

# Identify

## Source text

<!-- SRD page 142 -->

Level 1 Divination (Bard, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Touch\
Components: V, S, M (a pearl worth 100+ GP)\
Duration: Instantaneous\
You touch an object throughout the spell’s casting.\
If the object is a magic item or some other magi-\
cal object, you learn its properties and how to use\
them, whether it requires Attunement, and how\
many charges it has, if any. You learn whether any\
ongoing spells are affecting the item and what they\
are. If the item was created by a spell, you learn that\
spell’s name.\
If you instead touch a creature throughout the\
casting, you learn which ongoing spells, if any, are\
currently affecting it.\

Original layout: [[../../srd-5.2.1/pages/142|page 142]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 1 minute or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a pearl worth 100+ GP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/identify ` |
| ` implementation ` | ` dnd2024:spell_identify ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` kind_choices ` | ` ["object", "creature"] ` |
| ` touch_throughout_casting ` | ` true ` |

#### Knowledge

<!-- splat:values path=splat_rule/parameters/knowledge -->

| Field | Value |
| --- | --- |
| ` magical_object ` | ` ["properties", "use_instructions", "attunement_required", "charges_if_any"] ` |
| ` object ` | ` ["ongoing_spells", "creating_spell_name_if_created_by_spell"] ` |
| ` creature ` | ` ["ongoing_spells"] ` |
