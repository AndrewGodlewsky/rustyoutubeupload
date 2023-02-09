import random


def titler():
    titles = ["Minecraft", "This Game", "Video Games", "Xbox"]
    title1 = random.choice(titles)

    nouns = [
        "Creeper",
        "Alex",
        "Steve",
        "Wolf",
        "Cobble Stone",
        "Diamond Sword",
        "Chest",
    ]
    noun1 = random.choice(nouns)
    noun2 = random.choice(nouns)

    verbs = [
        "Fly",
        "Run",
        "Play",
        "Cry",
        "Laugh",
        "Dig",
        "Mine",
        "Explode",
        "Rest",
        "Fight",
        "Build",
    ]
    verb1 = random.choice(verbs)
    verb2 = random.choice(verbs)

    adjectives = [
        "Beautiful",
        "Crazy",
        "Happy",
        "Lucky",
        "Ugly",
        "Amazing",
        "Awesome",
        "Horrible",
        "Epic",
    ]
    adjective1 = random.choice(adjectives)
    adjective2 = random.choice(adjectives)
    adjective3 = random.choice(adjectives)

    interjections = [
        "Wow",
        "Ouch",
        "Ugh",
        "Yippee",
        "Oh well",
        "Oh no",
        "Dang",
        "Holy Smokes",
        "Hurray",
        "Holy Cow",
    ]
    interjection1 = random.choice(interjections)
    interjection2 = random.choice(interjections)

    title = random.choice(
        [
            f"Minecraft: The {noun1} update!",
            f"{interjection1}! {title1} is {adjective2}",
            f"{title1} is {verb1}ing",
            f"My {adjective1} Minecraft base is {adjective2}",
            f"That {noun1} is {verb1}ing!",
            f"I love {title1}. This is {adjective1}",
            f"{interjection1}, I am so {adjective2}",
            f"The {noun1} is better than {noun2}",
            f"Wanna join my {adjective1} server?",
            f"Let's {verb1} in Minecraft!",
            f"{interjection1}, this is more {adjective2} than I thought",
            f"{verb1}! There's a {noun1} coming"
        ]
    )
    return title


if False:
    print(titler())

else:
    for i in range(0, 100):
        a = titler()
        # if a.__contains__("There a"):
        #     print(a)
        
        print(a)
