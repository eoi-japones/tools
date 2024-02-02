import os
import yaml
from pymongo import MongoClient
import pymongo

mongo_uri = "mongodb://localhost:27017/"
database_name = "kanjis"
collection_name = "kanji"

client = MongoClient(mongo_uri)
db = client[database_name]
collection = db[collection_name]

carpeta_kanjis = "RUTA_A_kanji-data/data/"

for archivo_yaml in os.listdir(carpeta_yaml):
    if archivo_yaml.endswith(".yaml") or archivo_yaml.endswith(".yml"):
        ruta_completa = os.path.join(carpeta_yaml, archivo_yaml)

        try: 
            with open(ruta_completa, "r") as archivo:
                data = yaml.safe_load(archivo)
        except yaml.scanner.ScannerError:
            print("ERROR en el yaml: {}".format(ruta_completa))
            continue

        data["_id"] = data.pop("id")
        try: collection.insert_one(data)
        except UnicodeEncodeError: 
            print("ERROR de unicode en {}".format(ruta_completa))
            continue
        except pymongo.errors.DuplicateKeyError:
            print("ERROR id duplicado en {}".format(ruta_completa))
            continue

client.close()