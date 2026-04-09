//! Artifact Layer — deliberation produces adaptive executable artifacts
//! An artifact carries its provenance, confidence, and self-modification capability.

use std::collections::HashMap;

/// A provenance trace entry — records what each agent contributed
#[derive(Debug, Clone)]
pub struct ProvenanceEntry {
    pub agent: String,
    pub operation: String,
    pub confidence: f64,
    pub timestamp_nanos: u64,
    pub summary: String,
}

/// An artifact is the output of deliberation — a running program with metadata
#[derive(Debug, Clone)]
pub struct Artifact {
    /// The generated code/content
    pub code: String,
    /// Aggregate confidence from deliberation
    pub confidence: f64,
    /// Full provenance chain
    pub provenance: Vec<ProvenanceEntry>,
    /// Which agents participated
    pub agents_involved: Vec<String>,
    /// How many deliberation rounds
    pub rounds: usize,
    /// Self-modification policy
    pub adaptation_policy: AdaptationPolicy,
    /// Constraints this artifact satisfies
    pub constraints_satisfied: Vec<String>,
    /// Constraints this artifact violates
    pub constraints_violated: Vec<String>,
}

/// How the artifact adapts at runtime
#[derive(Debug, Clone)]
pub struct AdaptationPolicy {
    pub enabled: bool,
    pub monitor_interval_secs: u64,
    pub confidence_floor: f64,
    pub max_adaptations_per_hour: u32,
    pub auto_rollback_on_failure: bool,
}

impl Default for AdaptationPolicy {
    fn default() -> Self {
        Self {
            enabled: true, monitor_interval_secs: 60,
            confidence_floor: 0.5, max_adaptations_per_hour: 10,
            auto_rollback_on_failure: true,
        }
    }
}

impl Artifact {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(), confidence: 0.0,
            provenance: vec![], agents_involved: vec![],
            rounds: 0, adaptation_policy: AdaptationPolicy::default(),
            constraints_satisfied: vec![], constraints_violated: vec![],
        }
    }

    pub fn with_confidence(mut self, conf: f64) -> Self {
        self.confidence = conf.clamp(0.0, 1.0);
        self
    }

    pub fn add_provenance(&mut self, agent: &str, op: &str, confidence: f64, summary: &str) {
        self.provenance.push(ProvenanceEntry {
            agent: agent.to_string(), operation: op.to_string(),
            confidence, timestamp_nanos: now_nanos(), summary: summary.to_string(),
        });
        if !self.agents_involved.contains(&agent.to_string()) {
            self.agents_involved.push(agent.to_string());
        }
    }

    /// Check if artifact meets minimum confidence for deployment
    pub fn is_deployable(&self, threshold: f64) -> bool {
        self.confidence >= threshold && self.constraints_violated.is_empty()
    }

    /// Generate a trace report
    pub fn trace(&self) -> String {
        let mut lines = vec![format!("Artifact (confidence: {:.3}, {} rounds)", self.confidence, self.rounds)];
        lines.push(format!("Agents: {}", self.agents_involved.join(", ")));
        if !self.constraints_satisfied.is_empty() {
            lines.push(format!("Satisfied: {}", self.constraints_satisfied.join(", ")));
        }
        if !self.constraints_violated.is_empty() {
            lines.push(format!("VIOLATED: {}", self.constraints_violated.join(", ")));
        }
        lines.push("---".to_string());
        for e in &self.provenance {
            lines.push(format!("  [{}] {} {} (conf={:.3}) {}", e.agent, e.operation, e.summary, e.confidence, ""/*timestamp*/));
        }
        lines.push(format!("Code: {}{}", &self.code[..self.code.len().min(80)], if self.code.len() > 80 { "..." } else { "" }));
        lines.join("\n")
    }

    /// Create a checkpoint for rollback
    pub fn checkpoint(&self) -> ArtifactCheckpoint {
        ArtifactCheckpoint {
            code: self.code.clone(), confidence: self.confidence,
            provenance_len: self.provenance.len(),
        }
    }

    /// Rollback to a checkpoint
    pub fn rollback(&mut self, checkpoint: &ArtifactCheckpoint) {
        self.code = checkpoint.code.clone();
        self.confidence = checkpoint.confidence;
        self.provenance.truncate(checkpoint.provenance_len);
    }
}

/// A saved state for rollback
#[derive(Debug, Clone)]
pub struct ArtifactCheckpoint {
    pub code: String,
    pub confidence: f64,
    pub provenance_len: usize,
}

/// Artifact registry — tracks all artifacts in a system
pub struct ArtifactRegistry {
    artifacts: HashMap<String, Artifact>,
    deployments: HashMap<String, u64>,
}

impl ArtifactRegistry {
    pub fn new() -> Self { Self { artifacts: HashMap::new(), deployments: HashMap::new() } }

    pub fn register(&mut self, name: &str, artifact: Artifact) {
        self.artifacts.insert(name.to_string(), artifact);
    }

    pub fn deploy(&mut self, name: &str) -> Option<u64> {
        let artifact = self.artifacts.get(name)?;
        if artifact.is_deployable(0.7) {
            let count = self.deployments.entry(name.to_string()).or_insert(0);
            *count += 1;
            return Some(*count);
        }
        None
    }

    pub fn best_artifact(&self) -> Option<(&str, &Artifact)> {
        self.artifacts.iter().max_by(|a, b| a.1.confidence.partial_cmp(&b.1.confidence).unwrap())
    }
}

fn now_nanos() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_creation() {
        let a = Artifact::new("return sorted(data)").with_confidence(0.9);
        assert!((a.confidence - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_provenance() {
        let mut a = Artifact::new("fn()");
        a.add_provenance("architect", "propose", 0.8, "builtin sorted");
        assert_eq!(a.agents_involved.len(), 1);
        assert_eq!(a.provenance.len(), 1);
    }

    #[test]
    fn test_deployable() {
        let mut a = Artifact::new("fn()").with_confidence(0.8);
        assert!(a.is_deployable(0.7));
        a.constraints_violated.push("missing desc".to_string());
        assert!(!a.is_deployable(0.7));
    }

    #[test]
    fn test_checkpoint_rollback() {
        let mut a = Artifact::new("v1").with_confidence(0.8);
        a.add_provenance("a", "op", 0.8, "s");
        let cp = a.checkpoint();
        a.code = "v2".to_string();
        a.confidence = 0.3;
        a.rollback(&cp);
        assert_eq!(a.code, "v1");
        assert!((a.confidence - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_registry() {
        let mut reg = ArtifactRegistry::new();
        reg.register("sort", Artifact::new("sorted()").with_confidence(0.9));
        assert!(reg.best_artifact().is_some());
        assert!(reg.deploy("sort").is_some());
    }
}
