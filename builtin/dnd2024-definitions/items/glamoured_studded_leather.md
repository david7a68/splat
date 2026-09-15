---
splat_schema: 1
splat_id: cf84bbbb-c777-5992-b207-faec02b2afa9
splat_kind: item
splat_key: dnd2024:items/glamoured_studded_leather
splat_name: Glamoured Studded Leather
splat_origin:
  key: dnd2024:reference/magic_item/glamoured_studded_leather
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223, 224
  url: https://www.dndbeyond.com/srd
---

# Glamoured Studded Leather

## Source text

<!-- SRD page 223 -->

Armor (Studded Leather Armor), Rare\
While wearing this armor, you gain a +1 bonus to\
Armor Class. You can also take a Bonus Action to\

<!-- SRD page 224 -->

cause the armor to assume the appearance of a nor-\
mal set of clothing or some other kind of armor. You\
decide what it looks like—including color, style, and\
accessories—but the armor retains its normal bulk\
and weight. The illusory appearance lasts until you\
use this property again or doff the armor.\

Original layout: [[../../srd-5.2.1/pages/223|page 223]], [[../../srd-5.2.1/pages/224|page 224]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/glamoured_studded_leather ` |
| ` implementation ` | ` dnd2024:item_glamoured_studded_leather ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` ac_bonus ` | ` 1 ` |
| ` attunement_required ` | ` false ` |

#### Appearance change

<!-- splat:yaml path=splat_rule/parameters/appearance_change -->

```yaml
action: bonus_action
illusion: true
appearance:
  choice: wearer
  kinds:
  - normal_clothing
  - armor
  attributes:
  - color
  - style
  - accessories
retains:
- bulk
- weight
ends_on:
- next_appearance_change
- armor_doffed
```
