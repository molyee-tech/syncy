use log::*;

pub struct Node<RT> {
    rt: RT,
    swarm: Swarm<Behaviour>,
}

impl<RT> Node<RT> {
    pub fn listen<A: Iterator<Item = Multiaddr>(&mut self, addrs: A) -> Result<()> {
        for addr in addrs {
            info!("Start listening {:?} ..", addr);
            Swarm::listen_on(&mut self.swarm, addr)?;
        }
        Ok(())
    }
}