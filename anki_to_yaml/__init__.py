# -*- coding: utf-8 -*-
# Copyright: Ankitects Pty Ltd and contributors
# License: GNU GPL, version 3 or later; http://www.gnu.org/copyleft/gpl.html
#
# Exports the cards in the current deck to a HTML file, so they can be
# printed. Card styling is not included. Cards are printed in sort field
# order.

from __future__ import annotations

import re

from anki.cards import CardId
from anki.decks import DeckId
from anki.utils import ids2str
from aqt import mw
from aqt.qt import *
from aqt.utils import mungeQA, openLink

config = mw.addonManager.getConfig(__name__)


def sortFieldOrderCids(did: DeckId) -> list[CardId]:
    dids = [did]
    for name, id in mw.col.decks.children(did):
        dids.append(id)

    return mw.col.db.list(
        """
select c.id from cards c, notes n where did in %s
and c.nid = n.id order by n.sfld"""
        % ids2str(dids)
    )


def onPrint() -> None:
    ids = sortFieldOrderCids(mw.col.decks.selected())

    def normalize_str(string_to_normalize: str) -> str:
        # Remove capital case
        normalized_string = string_to_normalize.lower()

        # Remove punctuation (everything but characters and spaces)
        normalized_string = re.sub(r"[^\w\s]", "", normalized_string)

        # Replace spaces with underscores
        normalized_string = normalized_string.replace(" ", "_")

        return normalized_string

    def get_card_data(card) -> Dictionary:
        data = {}

        data["id"] = card.answer_text[
            card.answer_text.find("<rb>") + len("<rb>"):card.answer_text.find("</rb>")
        ]
        if len(data["id"]) > 1:
            KANJI_DIV_START = "<div class='kanji'>"
            data["id"] = card.answer_text[
                card.answer_text.find(KANJI_DIV_START) + len(KANJI_DIV_START):
                card.answer_text.find("</div>")
            ]
            if len(data["id"]) > 1:
                KANJI_SPAN_START = '<span style="color: rgb(170, 0, 0);">'
                data["id"] = card.answer_text[
                    card.answer_text.find(KANJI_SPAN_START) + len(KANJI_SPAN_START):
                    card.answer_text.find("</span>")
                ]
                if len(data["id"]) > 1:
                    starting_position_span_start = card.answer_text.find(KANJI_SPAN_START) + len(KANJI_SPAN_START)
                    starting_position_span_end = card.answer_text.find("</span>", starting_position_span_start) + len("</span>")
                    data["clave"] = normalize_str(data["id"])
                    data["id"] = card.answer_text[
                        card.answer_text.find(
                            KANJI_SPAN_START, starting_position_span_start
                        ) + len(KANJI_SPAN_START) :
                        card.answer_text.find("</span>", starting_position_span_end)
                    ]

        data_splitted = card.answer_text.split("<hr>")
        if(len(data_splitted) > 2):
            data["historia"] = data_splitted[2].strip()
            data["componentes"] = data_splitted[1].strip().split(" + ") if len(data_splitted[1]) else []
            data["como_componente"] = "[]"
        else:
            data["historia"] = ""
            data["componentes"] = ""
            data["como_componente"] = ""

        return data

    for j, cid in enumerate(ids):
        c = mw.col.get_card(cid)
        if c.ord == 1:
            card = c.render_output(True, False)
            normalized_card_name = normalize_str(card.question_text)

            folder_path = os.path.join(
                QStandardPaths.writableLocation(
                    QStandardPaths.StandardLocation.DesktopLocation
                ),
                "kanjis_{0}".format(mw.col.decks.selected())
            )
            if not os.path.exists(folder_path):
                os.mkdir(folder_path)

            card_data = get_card_data(card)
            if not card_data.get("clave", False):
                card_data["clave"] = normalized_card_name
            path = os.path.join(folder_path, "{0},{1}.yml".format(
                card_data["id"], card_data["clave"]
            ))
            buf = open(path, "w+", encoding="utf8")
            buf.write(
"""id: {0}
clave: {1}
historia: >
    {2}
componentes: {3}
como_componente: {4}""".format(
    card_data["id"],
    card_data["clave"],
    card_data["historia"],
    card_data["componentes"],
    card_data["como_componente"],
)
            )
            buf.close()


q = QAction(mw)
q.setText("To YAML")
q.setShortcut(QKeySequence("Shift+Y"))
mw.form.menuTools.addAction(q)
q.triggered.connect(onPrint)  # type: ignore

