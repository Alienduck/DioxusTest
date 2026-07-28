Phase 1 : L'Éveil (0 à 2 secondes)
- Action UI : Pendant que le texte de Dioxus attend ses 2 secondes avant de s'animer, la scène 3D est plongée dans une pénombre bleutée.
- Action 3D : La lune est de dos ou à contre-jour. Le composant EmissiveMaterial démarre à une intensité de 0.0.
- Transition : L'intensité émissive monte progressivement (via un fondu mathématique) pour atteindre son apogée exactement au moment où les lettres "Lucid Games" commencent à s'afficher à l'écran.

Phase 2 : La Lévitation (Boucle continue)
- Le Mouvement : Pour un effet "rêve", le mouvement doit être organique. On peut combiner la rotation avec un léger flottement vertical continu en utilisant une onde sinusoïdale (time.elapsed_secs().sin()).
- L'Éclairage : Remplace ta lumière blanche basique par deux PointLight distinctes : une lumière principale violette intense d'un côté, et une lumière de remplissage bleue plus douce de l'autre. Le modèle accrochera ces deux teintes sur ses reliefs.

Phase 3 : L'Écosystème Onirique (Le détail qui tue)
- Poussière d'étoiles : Utilise ton générateur aléatoire ChaCha8Rng pour faire apparaître une vingtaine de minuscules cubes ou sphères émissives autour de la lune.
- La touche studio : Pour donner encore plus de cachet, Mouzz pourrait modéliser de petits éclats de cristaux ou des fragments de roches low-poly dans Blender. Faire graviter doucement ces éléments personnalisés autour de la lune donnera immédiatement un aspect professionnel et fini à la landing page.
