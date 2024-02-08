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

        data["clave"] = card.answer_text[
            card.answer_text.find("<rb>") + 4:card.answer_text.find("</rb>")
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
            path = os.path.join(folder_path, "{0}.yml".format(
                normalized_card_name
            ))
            buf = open(path, "w+", encoding="utf8")
            buf.write(
"""id: {0}
clave: {1}
historia: >
    {2}
componentes: {3}
como_componente: {4}""".format(
    normalized_card_name,
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

