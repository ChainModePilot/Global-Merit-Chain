# Capítulo 5: Sistema de Identidad

## 5.1 Por qué se necesita un sistema de identidad dedicado

La identidad en GMC difiere de las cuentas tradicionales de internet:

- Se vincula a la reputación de por vida de una persona natural y no puede crearse ni descartarse arbitrariamente
- Debe soportar la vinculación permanente de iFay y la transferencia de propiedad de coFay
- Debe ser verificable en un entorno descentralizado mientras protege la privacidad

## 5.2 Capas de identidad

```
┌─────────────────────────────────────┐
│  Capa 1: Identidad de persona       │  ← Única, de por vida
│           natural (HumanID)          │
├─────────────────────────────────────┤
│  Capa 2: Identidad Fay (FayID)      │  ← Emparejado con HumanID
├─────────────────────────────────────┤
│  Capa 3: Capa de activos            │  ← Vinculado a FayID
│           (MeritPocket)              │
└─────────────────────────────────────┘
```

### HumanID

- Globalmente único, identifica a una persona natural
- Un HumanID puede corresponder a múltiples FayIDs
- Válido de por vida, no puede darse de baja (pero puede entrar en estado de cementerio)

### FayID

- Globalmente único, identifica a un Fay
- Cada FayID está asociado con un MeritPocket
- El FayID de un iFay está vinculado permanentemente a un HumanID
- La propiedad del FayID de un coFay puede transferirse

## 5.3 Esquema de verificación en cadena

### Comparación de esquemas

| Esquema | Principio | Ventajas | Desventajas | Escenarios aplicables |
|---------|-----------|----------|-------------|----------------------|
| PKI (Par de claves pública-privada) | Verificación por firma de par de claves | Maduro, eficiente, descentralizado | Pérdida de clave privada = pérdida de identidad | Firmas básicas |
| DID (Identidad descentralizada) | Estándar W3C, documentos de identidad en cadena | Estandarizado, soporta recuperación de claves | Relativamente complejo | Mapeo de relaciones |
| ZKP (Prueba de conocimiento cero) | Demuestra identidad sin revelar información | Protección de privacidad extremadamente fuerte | Alto costo computacional | Escenarios de privacidad |

### Recomendación: Combinación por capas

1. **Capa base (verificación básica)**: PKI
   - Mecanismo de firma para todas las operaciones en cadena
   - Cada HumanID y FayID tiene un par de claves

2. **Capa intermedia (gestión de relaciones)**: DID
   - Gestiona las relaciones de vinculación HumanID ↔ FayID
   - Soporta rotación de claves y recuperación social
   - Almacena metadatos de identidad

3. **Capa superior (escenarios de privacidad)**: ZKP
   - Demuestra identidad durante la votación sin revelar quién eres
   - Verifica relaciones durante la autenticación de herencia sin exponer detalles
   - Protege a los denunciantes durante quejas de sanción

### Justificación

> Cada esquema individual tiene limitaciones:
> - PKI puro no puede resolver la pérdida de claves y carece de protección de privacidad
> - DID puro tiene rendimiento insuficiente para verificación de alta frecuencia
> - ZKP puro tiene costos computacionales excesivos
>
> Una combinación por capas permite que cada capa se enfoque en los escenarios que mejor maneja.

## 5.4 Ciclo de vida de iFay

```
Creación → Vinculación al arquetipo humano → Operación normal → [Fallecimiento del arquetipo humano] → Tutela / Cementerio digital
```

### Operación normal

- iFay actúa en nombre del arquetipo humano
- Todo el MeriToken generado pertenece al arquetipo humano
- El arquetipo humano participa en votaciones, reconocimiento de contribuciones, etc. a través de iFay

### Tutela

Cuando el arquetipo humano fallece:
- Un heredero puede solicitar convertirse en tutor
- El tutor puede gestionar en nombre del fallecido, pero **no puede actuar bajo la identidad del arquetipo humano**
- Todas las acciones de tutela deben mostrar la información del tutor
- Existe un marcador explícito de tutela en cadena

### Cementerio digital

- Un iFay puede seguir teniendo interacciones pasivas después de ser colocado en el cementerio
- Todas las interacciones se etiquetan como "desde el cementerio digital"
- No se genera activamente nuevo MeriToken
- El MeriToken existente continúa decayendo normalmente

## 5.5 Transferencia de propiedad de coFay

Como activo, coFay sigue estas reglas de transferencia:

1. El MeritPocket se transfiere con el coFay; el MeriToken no se atenúa
2. Los registros de transferencia se almacenan en cadena; el historial de cambios de propiedad es inalterable
3. La transferencia requiere confirmación de firma de ambas partes
4. La continuidad del poder de voto del coFay no se ve afectada por la transferencia

## 5.6 Prevención de ataques Sybil

Una persona con múltiples cuentas es una amenaza clásica para los sistemas de identidad descentralizados:

- El registro de HumanID requiere una prueba de unicidad (método específico por determinar)
- Análisis del grafo social: Los usuarios reales tienen redes sociales naturales; las cuentas falsas exhiben patrones anormales
- Análisis de patrones de comportamiento: Múltiples cuentas controladas por la misma persona comparten características de comportamiento similares
- Confianza progresiva: Los permisos e influencia de los nuevos usuarios se liberan gradualmente

## 5.7 Notas de discusión

> Compromisos centrales en el sistema de identidad:
> - Seguridad vs. usabilidad: La verificación de tres capas aumenta la seguridad pero también la complejidad
> - Privacidad vs. transparencia: ZKP protege la privacidad; los registros en cadena aseguran la transparencia
> - Permanencia vs. flexibilidad: La vinculación permanente de iFay asegura que la reputación sea inseparable de la persona; la transferibilidad de coFay asegura la flexibilidad comercial
> - La prevención de ataques Sybil es un desafío eterno para la identidad descentralizada y requiere una combinación de múltiples enfoques
