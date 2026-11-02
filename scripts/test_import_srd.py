import unittest

import import_srd as srd


def cell(left, width, text, semibold=False):
    return {'left': left, 'width': width, 'text': text, 'semibold': semibold}


def line(*cells):
    return {'cells': list(cells), 'text': ' '.join(c['text'] for c in cells)}


def fragment(left, top, text, width=40, semibold=False, gill=True, caption=False, size=14):
    return {'left': left, 'top': top, 'text': text, 'width': width, 'semibold': semibold,
            'gill': gill, 'caption': caption, 'size': size}


class PatchTests(unittest.TestCase):
    def test_patches_apply_only_on_their_own_page(self):
        self.assertEqual(srd.patched(340, 'Int 6 −2 2 Wis'), 'Int 6 −2 −2 Wis')
        self.assertEqual(srd.patched(339, 'Int 6 −2 2 Wis'), 'Int 6 −2 2 Wis')
        self.assertTrue(srd.patched(168, 'manipulating a simple tool,').endswith('a simple tool,'))
        self.assertTrue(srd.patched(168, 'telekinetic grip, such as manipulating a simple tool,').endswith('from a vial.'))


class JoinedTests(unittest.TestCase):
    def setUp(self):
        srd.ATTESTED.clear()
        srd.ATTESTED.update({'foot', 'wide', 'description', 'battle', 'field'})

    def test_printed_word_break_goes(self):
        self.assertEqual(srd.joined("the descrip-", "tion here"), "the description here")
        self.assertEqual(srd.joined('this descrip-', 'tion'), 'this description')

    def test_real_compound_keeps_its_hyphen(self):
        self.assertEqual(srd.joined('a 5-foot-', 'wide Line'), 'a 5-foot-wide Line')
        self.assertEqual(srd.joined('ten 10-foot-by-', '10-foot panels'), 'ten 10-foot-by-10-foot panels')

    def test_known_cut_word_joins(self):
        self.assertEqual(srd.joined('the battle-', 'field'), 'the battlefield')


class AttestTests(unittest.TestCase):
    def test_halves_of_a_cut_word_are_not_words(self):
        srd.attest([{'text': 'the bat-'}, {'text': 'tlements stand'}])
        self.assertIn('stand', srd.ATTESTED)
        self.assertNotIn('bat', srd.ATTESTED)
        self.assertNotIn('tlements', srd.ATTESTED)


class TableTests(unittest.TestCase):
    def test_joined_roll_and_result_split_into_two_cells(self):
        text = srd.table_markdown([
            line(cell(103, 86, '1d100 Effect', True)),
            line(cell(103, 310, '01–20 You cast a spell originating from the chosen')),
            line(cell(152, 263, 'point of origin.')),
        ])
        self.assertIn('| 1d100 | Effect |', text)
        self.assertIn('| 01–20 | You cast a spell originating from the chosen point of origin. |', text)

    def test_repeated_header_splits_each_pair(self):
        text = srd.table_markdown([
            line(cell(100, 80, '1d100 Plane', True), cell(300, 80, '1d100 Plane', True)),
            line(cell(100, 60, '01–05 Abyss'), cell(300, 80, '55–59 Gehenna')),
        ])
        self.assertIn('| 01–05 | Abyss | 55–59 | Gehenna |', text)

    def test_a_value_with_a_unit_stays_one_cell(self):
        text = srd.table_markdown([
            line(cell(100, 50, 'Material', True), cell(300, 50, 'Duration', True)),
            line(cell(100, 50, 'Gems'), cell(300, 50, '10 minutes')),
        ])
        self.assertIn('| Gems | 10 minutes |', text)

    def test_one_fragment_with_two_sentences_fills_two_columns(self):
        text = srd.table_markdown([
            line(cell(101, 30, 'Lever', True), cell(147, 20, 'Up', True), cell(482, 37, 'Down', True)),
            line(cell(114, 7, '1'), cell(147, 659, 'Legs extend, allowing the apparatus to walk and swim. Legs retract, reducing the apparatus’s Speed and Swim')),
            line(cell(481, 306, 'Speed to 0.')),
        ])
        self.assertIn('| 1 | Legs extend, allowing the apparatus to walk and swim. | Legs retract, reducing the apparatus’s Speed and Swim Speed to 0. |', text)


class WrappedRowTests(unittest.TestCase):
    def test_a_short_gap_continues_the_row(self):
        block = [line(cell(100, 40, 'Cover', True), cell(300, 40, 'Benefit', True)),
                 line(cell(100, 40, 'Three-'), cell(300, 40, '+5 bonus')),
                 line(cell(100, 40, 'Quarters'), cell(300, 40, 'to AC'))]
        for top, row in zip((10, 32, 51), block):
            row['top'] = top
        self.assertIn('| Three-Quarters | +5 bonus to AC |', srd.table_markdown(block))

    def test_a_list_with_no_header_keeps_its_rows(self):
        block = [line(cell(100, 40, 'Attack'), cell(300, 40, 'Dodge')),
                 line(cell(100, 40, 'Dash'), cell(300, 40, 'Help'))]
        for top, row in zip((10, 29), block):
            row['top'] = top
        text = srd.table_markdown(block)
        self.assertIn('| Attack | Dodge |', text)
        self.assertIn('| Dash | Help |', text)

    def test_a_footnote_is_not_a_row(self):
        text = srd.table_markdown([
            line(cell(100, 40, 'Name', True), cell(300, 40, 'Cost', True)),
            line(cell(100, 40, 'Rope'), cell(300, 40, '1 GP')),
            line(cell(100, 300, '*Halve the cost.')),
        ])
        self.assertTrue(text.endswith('| Rope | 1 GP |\n\n*Halve the cost.'), text)

    def test_values_in_one_fragment_fill_their_columns(self):
        text = srd.table_markdown([
            line(cell(100, 20, 'Lv', True), cell(200, 20, 'A', True), cell(260, 20, 'B', True), cell(320, 20, 'C', True)),
            line(cell(100, 20, '1'), cell(200, 140, '2 — —')),
        ])
        self.assertIn('| 1 | 2 | — | — |', text)

    def test_name_and_damage_in_one_fragment_split(self):
        text = srd.table_markdown([
            line(cell(92, 40, 'Name', True), cell(206, 60, 'Damage', True), cell(323, 60, 'Mastery', True)),
            line(cell(92, 186, 'Hand Crossbow  1d6 Piercing'), cell(323, 22, 'Vex')),
        ])
        self.assertIn('| Hand Crossbow | 1d6 Piercing | Vex |', text)


class HeaderRowTests(unittest.TestCase):
    def test_mixed_labels_on_both_sides_are_a_header(self):
        self.assertTrue(srd.header_row([
            cell(101, 46, 'Terrain', True), cell(204, 94, 'Maximum Pace', True), cell(343, 117, 'Encounter Distance'),
            cell(503, 70, 'Foraging DC'), cell(605, 80, 'Navigation DC'), cell(724, 60, 'Search DC')]))

    def test_plain_cells_only_on_the_right_are_another_table(self):
        self.assertFalse(srd.header_row([
            cell(100, 40, 'Spell', True), cell(200, 40, 'School', True), cell(300, 40, 'Special', True),
            cell(476, 70, 'Cure Wounds'), cell(600, 70, 'Abjuration'), cell(740, 7, 'C')]))


class FullWidthTests(unittest.TestCase):
    def test_header_row_across_the_gutter_is_a_block(self):
        fragments = [
            fragment(100, 600, 'Lever', semibold=True),
            fragment(500, 600, 'Down', semibold=True),
            fragment(100, 620, 'One'), fragment(500, 620, 'Two'),
            fragment(100, 640, 'Three'), fragment(500, 640, 'Four'),
        ]
        self.assertEqual(srd.full_width_blocks(fragments, 445), [(599, 641)])

    def test_tables_side_by_side_are_not_a_block(self):
        fragments = [
            fragment(100, 600, 'Item', semibold=True),
            fragment(500, 600, 'Ink Pen'),
            fragment(100, 620, 'Acid'), fragment(500, 620, 'Jug'),
            fragment(100, 640, 'Bell'), fragment(500, 640, 'Lamp'),
        ]
        self.assertEqual(srd.full_width_blocks(fragments, 445), [])

    def test_a_second_caption_means_two_tables(self):
        fragments = [
            fragment(500, 570, 'Fiend Spells', semibold=True, caption=True, size=16),
            fragment(100, 600, 'Spell', semibold=True),
            fragment(500, 600, 'Warlock Level', semibold=True),
            fragment(100, 620, 'Gate'), fragment(500, 620, '3'),
            fragment(100, 640, 'Weird'), fragment(500, 640, '5'),
        ]
        self.assertEqual(srd.full_width_blocks(fragments, 445), [])


if __name__ == '__main__':
    unittest.main()
