# coding: utf-8

import datetime
import unittest

import fastobo

# --- HeaderFrame -----------------------------------------------------------

class TestHeaderFrame(unittest.TestCase):

    type = fastobo.header.HeaderFrame

    def test_init(self):
        try:
            frame = fastobo.header.HeaderFrame([])
        except Exception:
            self.fail("could not create `HeaderFrame` instance")
        try:
            frame = fastobo.header.HeaderFrame()
        except Exception:
            self.fail("could not create `HeaderFrame` instance")
        try:
            frame = fastobo.header.HeaderFrame((
                fastobo.header.FormatVersionClause("1.2"),
                fastobo.header.SavedByClause("Martin Larralde"),
            ))
        except Exception:
            self.fail("could not create `HeaderFrame` instance from iterable")

    def test_init_type_error(self):
        # self.assertRaises(TypeError, self.type, 1) # SEGFAULTS
        self.assertRaises(TypeError, self.type, [1])
        self.assertRaises(TypeError, self.type, ["abc"])
        self.assertRaises(TypeError, self.type, "abc")

# --- HeaderClause ----------------------------------------------------------

class _TestUnquotedStringClause(object):

    type = NotImplemented

    def test_init(self):
        try:
            vc = self.type("1.2")
        except Exception:
            self.fail("could not create `{}` instance", self.type.__name__)

    def test_init_type_error(self):
        self.assertRaises(TypeError, self.type, 1)
        self.assertRaises(TypeError, self.type, [])

    def test_repr(self):
        x = self.type("abc")
        self.assertEqual(repr(x), "{}('abc')".format(self.type.__name__))

    def test_eq(self):
        x = self.type("1.2")
        self.assertEqual(x, self.type("1.2"))
        self.assertNotEqual(x, self.type("1.3"))


class TestFormatVersionClause(_TestUnquotedStringClause, unittest.TestCase):

    type = fastobo.header.FormatVersionClause

    def test_str(self):
        vc = fastobo.header.FormatVersionClause("1.2")
        self.assertEqual(str(vc), "format-version: 1.2")
        vc = fastobo.header.FormatVersionClause("x:y")
        self.assertEqual(str(vc), "format-version: x:y")

    def test_property_version(self):
        vc1 = self.type("1.2")
        self.assertEqual(vc1.version, "1.2")
        vc1.version = "1.3"
        self.assertEqual(vc1.version, "1.3")
        self.assertEqual(repr(vc1), "FormatVersionClause('1.3')")


class TestDataVersionClause(_TestUnquotedStringClause, unittest.TestCase):

    type = fastobo.header.DataVersionClause

    def test_str(self):
        x = self.type("4.0")
        self.assertEqual(str(x), "data-version: 4.0")

    def test_property_version(self):
        vc1 = self.type("1.2")
        self.assertEqual(vc1.version, "1.2")
        vc1.version = "1.3"
        self.assertEqual(vc1.version, "1.3")
        self.assertEqual(repr(vc1), "DataVersionClause('1.3')")


class TestDateClause(unittest.TestCase):

    type = fastobo.header.DateClause

    def test_init(self):
        try:
            vc = self.type(datetime.datetime.now())
        except Exception:
            self.fail("could not create `{}` instance", self.type.__name__)

    def test_init_type_error(self):
        self.assertRaises(TypeError, self.type, 1)
        self.assertRaises(TypeError, self.type, [])

    @unittest.expectedFailure
    def test_repr(self):
        now = datetime.datetime.now()
        x = self.type(now)
        self.assertEqual(repr(x), "{}({!r})".format(self.type.__name__, now))

    @unittest.expectedFailure
    def test_eq(self):
        now = datetime.datetime.now()
        x = self.type(now)
        self.assertEqual(x, self.type(now))
        self.assertNotEqual(x, self.type(datetime.datetime.now()))

    def test_str(self):
        then = datetime.datetime(2019, 4, 8, 16, 51)
        vc = self.type(then)
        self.assertEqual(str(vc), "date: 08:04:2019 16:51")

    @unittest.expectedFailure
    def test_property_version(self):
        now = datetime.datetime.now()
        vc1 = self.type(now)
        self.assertEqual(vc1.date, now)

        then = datetime.datetime(2019, 4, 8, 16, 51)
        vc1.date = then
        self.assertEqual(vc1.date, then)

        with self.assertRaises(TypeError):
            vc1.date = 1


class TestSavedByClause(_TestUnquotedStringClause, unittest.TestCase):

    type = fastobo.header.SavedByClause


class TestAutoGeneratedByClause(_TestUnquotedStringClause, unittest.TestCase):

    type = fastobo.header.AutoGeneratedByClause


class TestRemarkClause(_TestUnquotedStringClause, unittest.TestCase):

    type = fastobo.header.RemarkClause


class TestOntologyClause(_TestUnquotedStringClause, unittest.TestCase):

    type = fastobo.header.OntologyClause


class TestOwlAxiomsClause(_TestUnquotedStringClause, unittest.TestCase):

    type = fastobo.header.OwlAxiomsClause
