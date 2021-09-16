// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#include "Commandline/ConfigurationManager.hpp"
#include "Generators/IProfileGenerator.hpp"
using namespace microsoft::quantum;

extern "C" void loadComponent(IProfileGenerator *generator);

class InlinerConfig
{
public:
  using String = std::string;

  void setup(ConfigurationManager &config)
  {
    config.setSectionName("Inliner component", "Adds the LLVM Always Inline Pass to the profile");
    config.addParameter(inline_, "custom-inliner", "Activating the custom inliner.");
  }

  bool shouldInline() const
  {
    return inline_;
  }

private:
  bool inline_{false};  ///< Default behaviour is that we do not add the inliner pass
};

extern "C" void loadComponent(IProfileGenerator *generator)
{
  generator->registerProfileComponent<InlinerConfig>(
      "inliner", [](InlinerConfig const &cfg, IProfileGenerator *ptr, Profile & /*profile*/) {
        if (cfg.shouldInline())
        {
          auto &module_pass_manager = ptr->modulePassManager();

          // Adds the inline pipeline
          auto &pass_builder = ptr->passBuilder();
          auto  inliner_pass = pass_builder.buildInlinerPipeline(
              ptr->optimisationLevel(), llvm::PassBuilder::ThinLTOPhase::None, ptr->debug());
          module_pass_manager.addPass(std::move(inliner_pass));
        }
      });
}
