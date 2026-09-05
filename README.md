# Bhaskara-RS

<p align="left">
  <strong>Motor analítico para equações de segundo grau desenvolvido em Rust</strong>
</p>

---

## Sumário
* [Visão Geral](#visão-geral)
* [Arquitetura do Código](#arquitetura-do-código)
* [Análise Matemática](#análise-matemática)
* [Demonstração de Uso](#demonstração-de-uso)
* [Instalação e Execução](#instalação-e-execução)

---

## Visão Geral

> ### Nota de Arquitetura
> O projeto foi desenvolvido sob os conceitos de **Clean Code** e separação estrita de escopo, garantindo que operações matemáticas puras fiquem completamente isoladas de efeitos colaterais de entrada e saída (I/O).

Este software recebe os coeficientes de uma equação polinomial quadrática e realiza a extração completa de suas propriedades geométricas e algébricas básicas.

---

## Arquitetura do Código

A estrutura do projeto reflete os padrões idiomáticos do ecossistema Rust, dividindo o ciclo de vida do dado em fases bem definidas:

| Componente | Responsabilidade |
| :--- | :--- |
| `main.rs` | Gerenciamento de fluxo, captura de argumentos e renderização em console. |
| `bhaskara.rs` | Definição da estrutura `Squared` e motor privado de cálculo numérico. |
| `std::fmt::Display` | Camada de formatação nativa para tratamento estético de sinais algébricos. |

```rust
// Abstração da estrutura de dados principal
pub struct Squared {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub x1: f64,
    pub x2: f64,
    pub xv: f64,
    pub yv: f64,
}
```

---

## Análise Matemática

O motor executa três pipelines matemáticos em funções de responsabilidade única:

```ini
[1. Discriminante] -> Cálculo de Delta com escape seguro para números não-reais (NaN).
[2. Raízes Reais]  -> Aplicação da fórmula de Bhaskara para interseções do eixo X.
[3. Vértice]       -> Determinação das coordenadas críticas de máximo e mínimo.
```

---

## Demonstração de Uso

Ao instanciar a equação com valores de ponto flutuante de dupla precisão, a saída estruturada no console é gerada através da formatação customizada:

```text
========================================
          RESULTADOS DA EQUACAO         
========================================
Equacao resolvida: 1x² -3.5x +3= 0
Raiz 1 (x1)   : 2.0000
Raiz 2 (x2)   : 1.5000
Vertice (xv)  : 1.7500
Vertice (yv)  : -0.2500
========================================
```

---

## Instalação e Execução

Para clonar e executar este projeto localmente, utilize o fluxo de comandos padrão do gerenciador de pacotes do Rust:

```bash
# 1. Clonar o repositório
git clone https://github.com

# 2. Navegar até o diretório do projeto
cd bhaskara-rs

# 3. Compilar e rodar em modo de desenvolvimento
cargo run
```

---
<p align="center">
  <sub>Bhaskara-RS • Desenvolvido em conformidade com as boas práticas de engenharia de software.</sub>
</p>
