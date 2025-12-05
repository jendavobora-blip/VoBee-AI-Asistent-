# Kompletní design a sdílení: VoBee AI Asistent

## Vizuální identita
- **Pozadí:** Midnight Blue #0B1026
- **Akcent:** Gold #D4AF37
- **Panic/Zen:** #1E3A8A
- **Typografie:** Inter (system-ui)
- **Styl komponent:** Glassmorphism karty (bg-black/20, backdrop-blur-sm), radius 16 px, soft shadow, lucide-react ikony

## Avatar a stavy
- **Portrét:** dle tvé fotky (zlatý rám, čisté pozadí)
- **Emoji nálady:** calm 🙂 | curious 🧐 | stressed 🧘 | panic 🫶
- **isPrivacyMode:** rozostření částek/procent/grafů + banner
- **isPanicMode:** Zen Blue paleta, potlačení červených prvků, uklidňující checklist
- **userMood:** upraví výraz avatara a tón odpovědi

## Wireframe (mobil)
┌─────────────────────────────────────────────┐
│ Header  [VoBee AI Asistent]   [Eye/EyeOff] │
│ AI loga: ◼ ◼ ◼ ◼ ◼ ◼                      │
├─────────────────────────────────────────────┤
│ [Portrét + emoji]                          │
│ „Ahoj, jsem VOBee. Jdeme klidně a chytře." │
│ [Spustit asistenta]  [PANIKA / TRH PADÁ]   │
├─────────────────────────────────────────────┤
│ Edu karty: Bitcoin & cykly / Čistá ruka /  │
│ S&P 500 / Banky vs inflace [Play]          │
├─────────────────────────────────────────────┤
│ FOMO Checker: „Nakupuješ blízko ATH?"      │
│ [Ověřit plán]                              │
├─────────────────────────────────────────────┤
│ Chat: [🎤] Zeptej se na cokoliv... [Send]  │
└─────────────────────────────────────────────┘

## Wireframe (desktop)
┌──────────────────────────────────────────────────────────────────────┐
│ Header [Logo] | AI: ◼ ◼ ◼ ◼ ◼ ◼ | [Privacy Eye]                     │
├──────────────────────────────────────────────────────────────────────┤
│ [Avatar + zlatý rám + emoji]   │ [Spustit] [PANIKA]                 │
│ „Jdeme klidně. Co řešíš?"      │ EduCards 2x2 + Play                │
│                                │ FOMO Checker + plán                │
├──────────────────────────────────────────────────────────────────────┤
│ Chat bubliny (assistant vlevo, user vpravo) + Play u odpovědí        │
└──────────────────────────────────────────────────────────────────────┘

## Interakce
- **Panic Button:** Zen Blue, skrývá červené prvky, checklist (Zastavit / Zhodnotit / Naplánovat)
- **Privacy Toggle:** blur citlivých údajů + banner
- **Audio Play:** hlasová edukace (20–40 s) + mini equalizer
- **FOMO Checker:** varování při ATH, CTA „Vytvořit krátký plán"

## Microcopy (starší brácha)
- Uvítání: „Ahoj, jsem VOBee. Projdeme to klidně a chytře. Co tě pálí?"
- Privacy: „Jsi na veřejnosti? Schovávám citlivá čísla."
- Panic: „Dýcháme. Teď nic neprodáváme narychlo."
- FOMO: „Bez plánu je to loterie. Dáme si 24 hodin pauzu."

## 500 otázek – bloky
1. Finanční gramotnost – 60 Q
2. Investice – základy – 50 Q
3. Akcie – 50 Q
4. ETF – 50 Q
5. Kryptoměny – 60 Q
6. Osobní finance – pokročilé – 40 Q
7. Psychologie investora – 40 Q
8. Globální ekonomika a trendy – 50 Q
9. Praktické scénáře a simulace – 50 Q
Celkem: 500 Q

## Supabase schéma
- users(id, email, display_name, privacy_default, created_at)
- settings(user_id, isPrivacyMode, isPanicMode, userMood, updated_at)
- chat_history(id, user_id, role, content, audio_url?, created_at)
- quiz_results(id, user_id, block_id, correct, total, created_at)
- Storage: audio/voice, avatar assets, bannery
- Realtime: chat/mood/quiz kanály

## Komponenty (props)
- HeaderBar(title, showPrivacyToggle, aiLogos[])
- AvatarCard(imageSrc, mood, privacyOn)
- PrimaryButton(label, onClick, variant)
- PanicButton(onActivate)
- EduCard(title, text, onPlay)
- FomoModal(open, onClose, onPlan)
- PanicModal(open, onClose)
- ChatBubble(role, text, audioUrl?)
- ChatInput(onSend, onMic)

## Spolupracující AI technologie (loga v grafice)
- 🔷 Gemini 3
- 🟦 Microsoft 365 Pro
- 🟣 Perplexity Pro
- 🟢 Copilot Clude 4.5
- 🔶 DeepSeek
- Claude
- Text v UI: „Na vývoji a udržitelnosti se podílí 6 umělých inteligencí"

## Ceník
- Free Trial: 0 Kč / 7 dní zdarma
- Premium Měsíční: 189 Kč / měsíc
- Premium Roční: 1799 Kč / rok (ušetříš ~21 %, ≈ 481 Kč)

## Poznámka k vývoji
- Aplikace se bude dál vyvíjet a rozšiřovat (nové moduly, hlasové profily, behavior checker, personalizace plánů).
